use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::Level;
use tracing_subscriber::EnvFilter;
use utils::config::LOADED_CONFIG;

pub async fn connect() -> DatabaseConnection {
    let filter = EnvFilter::from_default_env();
    let url = LOADED_CONFIG.database_url.clone();
    let mut connection_opts = ConnectOptions::new(url.clone());
    connection_opts.sqlx_logging(
        filter
            .max_level_hint()
            .map_or(false, |lvl| lvl >= Level::DEBUG),
    );
    connection_opts.sqlx_logging_level(log::LevelFilter::Debug);
    log::info!("Attempting to connect to database...");
    let connection = Database::connect(connection_opts)
        .await
        .inspect_err(|err| log::error!("Failed to connect to {}: {}", url, err))
        .expect("Failed to connect to the database!");
    log::info!("Connection OK, run migrations");
    migration::Migrator::up(&connection, None)
        .await
        .expect("Failed to execute migrations");
    log::info!("Migrations OK");
    connection
}
