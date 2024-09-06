use anyhow::{Context, Ok};
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool, PooledConnection}, SqliteConnection};
use tracing::info;

pub type ConnectionPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct Database {
    pub pool: ConnectionPool,
}

impl Database {
    pub async fn establish_connection_pool(database_url: &str) -> anyhow::Result<Self> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create a connection pool.");

        Ok(Self { pool })
    }
}