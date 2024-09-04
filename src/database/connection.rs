use anyhow::{Context, Ok};
use diesel::{prelude::*, sqlite::Sqlite};
use tracing::info;

/// SQLite database
pub struct Database {
    pub connection: SqliteConnection,
}

impl Database {
    pub async fn connect(database_url: &str) -> anyhow::Result<Self> {
        let connection = SqliteConnection::establish(&database_url)
            .context(format!("Failed to connect to \'{}\'", database_url))?;

        Ok(Self { connection })
    }
}