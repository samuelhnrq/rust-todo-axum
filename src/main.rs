use migration::MigratorTrait;
use std::error::Error;
use std::net::SocketAddr;

use axum::{extract::State, http::StatusCode, routing::get, Router};
use infra::tasks::controller::get_all_tasks;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use state::AppState;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::filter::EnvFilter;

mod infra;
mod model;
mod state;
mod tasks_repository;

#[axum_macros::debug_handler]
async fn ping(State(state): State<AppState>) -> (StatusCode, &'static str) {
    let ping_result = state.connection.ping().await;
    return match ping_result {
        Ok(_) => (StatusCode::OK, "OK"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database is down"),
    };
}

fn build_app(connection: DatabaseConnection) -> Router {
    let trace = TraceLayer::new_for_http();
    return Router::new()
        .route("/todos", get(get_all_tasks))
        // .route("/todos", post(create_new_todo))
        .route("/ping", get(ping))
        .layer(trace)
        .with_state(AppState { connection });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    // build our application with a single route
    log::info!("Initializing, connecting to the database");
    let db_url = std::env::var("DATABASE_URL").expect("Missing env variable DATABASE_URL");
    let filter = EnvFilter::from_default_env();
    let mut connection_opts = ConnectOptions::new(db_url);
    connection_opts.sqlx_logging(
        filter
            .max_level_hint()
            .map_or(false, |lvl| lvl >= Level::DEBUG),
    );
    connection_opts.sqlx_logging_level(log::LevelFilter::Debug);
    let connection = Database::connect(connection_opts).await?;
    log::info!("Connection OK, run migrations");
    migration::Migrator::up(&connection, None).await?;
    log::info!("Migrations OK, Will serve on 8080");
    let app = build_app(connection.clone());
    let service = app.into_make_service();
    let target_port: u16 =
        std::env::var("PORT").map_or(8080, |port_str| port_str.parse().expect("Invalid PORT env"));
    let bind_addr = SocketAddr::new("0.0.0.0".parse()?, target_port);
    axum::Server::bind(&bind_addr)
        .serve(service)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
            connection.close().await.unwrap();
        })
        .await
        .expect("Failed to bind on port 8080");
    log::info!("End");
    Ok(())
}
