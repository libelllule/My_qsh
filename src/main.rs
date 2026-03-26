use sqlx::{
    migrate::{MigrateDatabase, MigrateError}, sqlite::SqlitePoolOptions, Sqlite, SqlitePool,
};
use mac_address::MacAddress;
use std::fmt::Display;

const STANDARD_DB_URL: &str = "QSH.db";

const INSERT_TEMPLATE: &str = "INSERT INTO {table} ({columns}) VALUES {values};";
const VACUUM_TABLES: &str =
    "PRAGMA writable_schema = 1; DELETE FROM sqlite_master; PRAGMA writable_schema = 0; VACUUM; PRAGMA integrity_check;";

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
        let pool = self
            .pool
            .as_ref()
            .expect("pool is not initialized; call initialize_db first");
        Ok(sqlx::migrate!("./migrations").run(pool).await.map(|_| ())?)
    }

    pub async fn clear_database(&self) -> Result<(), sqlx::Error> {
        let pool = self
            .pool
            .as_ref()
            .expect("pool is not initialized; call initialize_db first");
        sqlx::query(VACUUM_TABLES).execute(pool).await.map(|_| ())
    }

    pub async fn insert_users(&self, users: &[impl Display]) -> Result<(), sqlx::Error> {
        let pool = self
            .pool
            .as_ref()
            .expect("pool is not initialized; call initialize_db first");
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
        let pool = self
            .pool
            .as_ref()
            .expect("pool is not initialized; call initialize_db first");
        let values = self.parse_items_to_values(notes).await;
        let query = INSERT_TEMPLATE
            .replace("{table}", "history")
            .replace("{columns}", "user_id, filename, size, date, status")
            .replace("{values}", &values);
        sqlx::query(&query).execute(pool).await.map(|_| ())
    }
}

#[tokio::main]
async fn main() {
    println!("This does nothing)))");
}

