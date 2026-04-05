use sqlx::{
    migrate::{MigrateDatabase, MigrateError}, sqlite::SqlitePoolOptions, Sqlite, SqlitePool,
    Row, sqlite::SqliteRow
};
use history::Note;
use user::User;

const STANDARD_DB_URL: &str = "QSH.db";
const POOL_NOT_INITIALIZED_ERR: &str = "pool is not initialized; call initialize_db first";

const SELECT_USERS_TEMPLATE: &str = "SELECT id, mac_address, device_name, nickname, status FROM users;";
const SELECT_NOTES_TEMPLATE: &str = "SELECT id, user_id, filename, size, date, status FROM history";

const INSERT_USER_QUERY: &str = "INSERT INTO users (mac_address, device_name, nickname, status) VALUES (?, ?, ?, ?)";
const INSERT_NOTE_QUERY: &str = "INSERT INTO history (user_id, filename, size, date, status) VALUES (?, ?, ?, ?, ?)";

const UPDATE_USER_QUERY: &str = "UPDATE users SET mac_address = ?, device_name = ?, nickname = ?, status = ? WHERE id = ?";
const UPDATE_NOTE_QUERY: &str = "UPDATE history SET user_id = ?, filename = ?, size = ?, date = ?, status = ? WHERE id = ?";

const DELETE_USER_BY_ID_QUERY: &str = "DELETE FROM users WHERE id = ?";
const DELETE_NOTE_BY_ID_QUERY: &str = "DELETE FROM history WHERE id = ?";

const VACUUM_TABLES: &str =
    "PRAGMA writable_schema = 1; DELETE FROM sqlite_master; PRAGMA writable_schema = 0; VACUUM; PRAGMA integrity_check;";
const END_CHECKPOINT: &str = "PRAGMA wal_checkpoint(TRUNCATE)";

pub struct BDHandler {
    db_url: Option<String>,
    pool: Option<SqlitePool>,
}

impl BDHandler {
    async fn check_pool(&self) -> &SqlitePool {
        self
            .pool
            .as_ref()
            .expect(POOL_NOT_INITIALIZED_ERR) 
    }

    async fn delete_from_table_by_id(&self, query: &str, id: u64) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        sqlx::query(query).bind(id.to_string()).execute(pool).await.map(|_| ())
    }

    async fn fetch_all<T>(&self, query: &str, 
        mapper: impl Fn(&SqliteRow) -> Option<T>) -> Vec<T> {
        let pool = self.check_pool().await;
        match sqlx::query(query).fetch_all(pool).await {
            Ok(rows) => rows
                .into_iter()
                .filter_map(|r| mapper(&r))
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn row_to_note(row: &SqliteRow) -> Option<Note> {
        let id: u64 = row.get("id");
        let user_id: u64 = row.get("user_id");
        let filename: String = row.get("filename");
        let size: String = row.get("size");
        let date: String = row.get("date");
        let status: String = row.get("status");
        Some(Note {
            id,
            user_id,
            filename,
            size,
            date,
            status,
        })
    }

    fn row_to_user(row: &SqliteRow) -> Option<User> {
        let id: u64 = row.get("id");
        let mac: String = row.get("mac_address");
        let device_name: String = row.get("device_name");
        let nickname: String = row.get("nickname");
        let status: String = row.get("status");
        User::constructor(&id, &mac, &device_name, &nickname, &status)
    }

    async fn insert_users_parameterized(&self, users: &[User]) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        
        let mut transaction = pool.begin().await?;
        
        for user in users {
            sqlx::query(INSERT_USER_QUERY)
                .bind(&user.mac_addr.to_string())
                .bind(&user.device_name)
                .bind(&user.nickname)
                .bind(&user.status)
                .execute(&mut *transaction)
                .await?;
        }
        
        transaction.commit().await?;
        Ok(())
    }

    async fn insert_notes_parameterized(&self, notes: &[Note]) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        
        let mut transaction = pool.begin().await?;
        
        for note in notes {
            sqlx::query(INSERT_NOTE_QUERY)
                .bind(note.user_id.to_string())
                .bind(&note.filename)
                .bind(&note.size)
                .bind(&note.date)
                .bind(&note.status)
                .execute(&mut *transaction)
                .await?;
        }
        
        transaction.commit().await?;
        Ok(())
    }

    pub async fn new() -> Self {
        BDHandler {
            db_url: Some(STANDARD_DB_URL.to_string()),
            pool: None,
        }
    }

    pub async fn initialize_db(&mut self) -> Result<(), sqlx::Error> {
        let db_url = self.db_url.as_deref().unwrap_or(STANDARD_DB_URL);

        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            let _ = Sqlite::create_database(db_url).await?;
        }

        if self.pool.is_none() {
            let pool = SqlitePoolOptions::new().connect(db_url).await?;
            self.pool = Some(pool);
        }
        Ok(())
    }

    pub async fn create_tables(&self) -> Result<(), MigrateError> {
        let pool = self.check_pool().await;
        Ok(sqlx::migrate!("./migrations").run(pool).await.map(|_| ())?)
    }

    pub async fn clear_database(&self) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        sqlx::query(VACUUM_TABLES).execute(pool).await.map(|_| ())
    }

    pub async fn insert_users(&self, users: &[User]) -> Result<(), sqlx::Error> {
        self.insert_users_parameterized(users).await
    }

    pub async fn insert_note(&self, notes: &[Note]) -> Result<(), sqlx::Error> {
        self.insert_notes_parameterized(notes).await
    }

    pub async fn update_user(&self, prev_user: User, new_user: User) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        sqlx::query(UPDATE_USER_QUERY)
            .bind(new_user.mac_addr.to_string())
            .bind(new_user.device_name)
            .bind(new_user.nickname)
            .bind(new_user.status)
            .bind(prev_user.id.to_string())
            .execute(pool)
            .await
            .map(|_| ())
    }

    pub async fn update_note(&self, prev_note: Note, new_note: Note) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        sqlx::query(UPDATE_NOTE_QUERY)
            .bind(new_note.user_id.to_string())
            .bind(new_note.filename)
            .bind(new_note.size)
            .bind(new_note.date)
            .bind(new_note.status)
            .bind(prev_note.id.to_string())
            .execute(pool)
            .await
            .map(|_| ())
    }

    pub async fn delete_user_by_id(&self, id: u64) -> Result<(), sqlx::Error> {
        self.delete_from_table_by_id(DELETE_USER_BY_ID_QUERY, id).await
    }

    pub async fn delete_note_by_id(&self, id: u64) -> Result<(), sqlx::Error> {
        self.delete_from_table_by_id(DELETE_NOTE_BY_ID_QUERY, id).await
    }

    pub async fn get_all_users(&self) -> Vec<User> {
        self.fetch_all(SELECT_USERS_TEMPLATE, BDHandler::row_to_user,).await
    }

    pub async fn get_all_notes(&self) -> Vec<Note> {
        self.fetch_all(SELECT_NOTES_TEMPLATE, BDHandler::row_to_note,).await
    }

    pub async fn close_pool(&mut self) {
        if let Some(pool) = self.pool.take() {
            let _ = sqlx::query(END_CHECKPOINT)
                .execute(&pool)
                .await;
            pool.close().await;
        }
    }
}
