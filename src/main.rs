use std::sync::Arc;
use dotenvy::dotenv;
use tracing::info;

use kotran::{Config, Database, Server, Logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Arc::new(Config::parse());

    let _guard = Logger::init(config.sys_env);

    let database = Database::establish_connection_pool(&config.database_url)
        .await
        .expect("Could not establish a database connection.");

    Server::serve(config, database)
        .await
        .context("Could not initialize an application server.")?;

    Ok(())
}
