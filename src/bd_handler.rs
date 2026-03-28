use sqlx::{
    migrate::{MigrateDatabase, MigrateError}, sqlite::SqlitePoolOptions, Sqlite, SqlitePool,
    Row, sqlite::SqliteRow
};
use mac_address::MacAddress;
use std::fmt::Display;

const STANDARD_DB_URL: &str = "QSH.db";
const POOL_NOT_INITIALIZED_ERR: &str = "pool is not initialized; call initialize_db first";

const SELECT_USERS_TEMPLATE: &str = "SELECT id, mac_address, device_name, nickname, status FROM users;";
const SELECT_NOTES_TEMPLATE: &str = "SELECT id, user_id, filename, size, date, status FROM history";
const INSERT_TEMPLATE: &str = "INSERT INTO {table} ({columns}) VALUES {values};";
const VACUUM_TABLES: &str =
    "PRAGMA writable_schema = 1; DELETE FROM sqlite_master; PRAGMA writable_schema = 0; VACUUM; PRAGMA integrity_check;";

const DELETE_TEMPLATE: &str = "DELETE FROM {table} WHERE {condition};";
const UPDATE_TEMPLATE: &str = "UPDATE {table} SET {values} WHERE {condition};";
const VALUES_USER_TEMPLATE: &str = "mac_address = {mac_addr}, device_name = {dev_name}, nickname = {nick}, status = {status}";
const VALUES_NOTE_TEMPLATE: &str = "user_id = {uid}, filename = {filename}, size = {size}, date = {date}, status = {status}";

pub struct User {
    pub id: u64,
    pub mac_addr: MacAddress,
    pub device_name: String,
    pub nickname: String,
    pub status: String,
}

pub struct Note {
    pub id: u64,
    pub user_id: u64,
    pub filename: String,
    pub size: String,
    pub date: String,
    pub status: String,
}

pub struct BDHandler {
    db_url: Option<String>,
    pool: Option<SqlitePool>,
}

impl User {
    pub async fn to_string(&self) -> String {
        format!("{}, {}, {}", self.mac_addr, self.device_name, self.nickname)
    }
}

impl Note {
    pub async fn to_string(&self) -> String {
        format!(
            "{}, {}, {}, {}, {}",
            self.user_id, self.filename, self.size, self.date, self.status
        )
    }
}

impl BDHandler {
    async fn check_pool(&self) -> &SqlitePool {
        self
            .pool
            .as_ref()
            .expect(POOL_NOT_INITIALIZED_ERR) 
    }

    async fn delete_from_table(&self, table: &str, condition: &str) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        let query = DELETE_TEMPLATE
            .replace("{table}", table)
            .replace("{condition}", condition);
        sqlx::query(&query).execute(pool).await.map(|_| ())
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
        match mac.parse::<MacAddress>() {
            Ok(mac_addr) => Some(User {
                id,
                mac_addr,
                device_name,
                nickname,
                status,
            }),
            Err(_) => None,
        }
    }

    async fn parse_items_to_values<T: Display>(&self, items: &[T]) -> String {
        let mut out = String::new();
        let mut first = true;
        for item in items {
            if first {
                first = false;
            } else {
                out.push_str(", ");
            }
            out.push('(');
            out.push_str(&item.to_string());
            out.push(')');
        }
        if out.is_empty() {
            out.push_str("()");
        }
        out
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

    pub async fn insert_users(&self, users: &[impl Display]) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        let values = self.parse_items_to_values(users).await;
        let query = INSERT_TEMPLATE
            .replace("{table}", "users")
            .replace(
                "{columns}",
                "mac_address, device_name, nickname, status",
            )
            .replace("{values}", &values);
        sqlx::query(&query).execute(pool).await.map(|_| ())
    }

    pub async fn insert_note(&self, notes: &[impl Display]) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        let values = self.parse_items_to_values(notes).await;
        let query = INSERT_TEMPLATE
            .replace("{table}", "history")
            .replace("{columns}", "user_id, filename, size, date, status")
            .replace("{values}", &values);
        sqlx::query(&query).execute(pool).await.map(|_| ())
    }

    pub async fn update_user(&self, prev_user: User, new_user: User) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        let query = UPDATE_TEMPLATE
            .replace("{table}", "users")
            .replace("{values}", VALUES_USER_TEMPLATE)
            .replace("{mac_addr}", &new_user.mac_addr.to_string())
            .replace("{dev_name}", &new_user.device_name)
            .replace("{nick}", &new_user.nickname)
            .replace("{status}", &new_user.status)
            .replace("{condition}", "id = {id}")
            .replace("{id}", &prev_user.id.to_string());
        sqlx::query(&query).execute(pool).await.map(|_| ())
    }

    pub async fn update_note(&self, prev_note: Note, new_note: Note) -> Result<(), sqlx::Error> {
        let pool = self.check_pool().await;
        let query = UPDATE_TEMPLATE
            .replace("{table}", "history")
            .replace("{values}", VALUES_NOTE_TEMPLATE)
            .replace("{uid}", &new_note.user_id.to_string())
            .replace("{filename}", &new_note.filename)
            .replace("{size}", &new_note.size)
            .replace("{date}", &new_note.date)
            .replace("{status}", &new_note.status)
            .replace("{condition}", "id = {id}")
            .replace("{id}", &prev_note.id.to_string());
        sqlx::query(&query).execute(pool).await.map(|_| ())
    }

    pub async fn delete_users(&self, condition: &str) -> Result<(), sqlx::Error> {
        self.delete_from_table("users", condition).await
    }

    pub async fn delete_notes(&self, condition: &str) -> Result<(), sqlx::Error> {
        self.delete_from_table("history", condition).await
    }

    pub async fn get_all_users(&self) -> Vec<User> {
        self.fetch_all(SELECT_USERS_TEMPLATE, BDHandler::row_to_user,).await
    }

    pub async fn get_all_notes(&self) -> Vec<Note> {
        self.fetch_all(SELECT_NOTES_TEMPLATE, BDHandler::row_to_note,).await
    }
}
