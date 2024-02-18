use std::error::Error;
use std::net::SocketAddr;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use infra::tasks::controller::{create_task, get_all_tasks};
use state::AppState;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::filter::EnvFilter;

mod infra;
mod model;
mod state;

#[axum_macros::debug_handler]
async fn ping(State(state): State<AppState>) -> (StatusCode, &'static str) {
    let ping_result = state.connection.ping().await;
    return match ping_result {
        Ok(_) => (StatusCode::OK, "OK"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database is down"),
    };
}

fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/todos", get(get_all_tasks))
        .route("/todos", post(create_task))
        .route("/ping", get(ping))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive("INFO".parse().unwrap())
                .from_env()
                .expect("Invalid log config"),
        )
        .init();
    // build our application with a single route
    log::info!("Initializing, connecting to the database");
    let state = AppState::new().await;
    let app = build_app(state.clone());
    let service = app.into_make_service();
    let target_port: u16 =
        std::env::var("PORT").map_or(8080, |port_str| port_str.parse().expect("Invalid PORT env"));
    log::info!("Trying to bind on port {}", target_port);
    let bind_addr = SocketAddr::new("0.0.0.0".parse()?, target_port);
    let listener = TcpListener::bind(bind_addr).await?;

    axum::serve(listener, service)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
            log::warn!("Shutting down");
            state.connection.close().await.unwrap();
        })
        .await
        .expect("Failed to bind on port 8080");
    log::info!("End");
    Ok(())
}
