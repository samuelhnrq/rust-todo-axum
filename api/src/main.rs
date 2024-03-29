use std::error::Error;
use std::net::SocketAddr;

use crate::adapters::{
    authentication::authentication_middleware,
    controllers::{
        tasks::{create_task, get_all_tasks},
        users::{create_user, get_all_users},
    },
    static_files::static_files_service,
};
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    routing::{get, post},
    Router,
};
use entity::AppState;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tower_http::trace::TraceLayer;
use tracing_subscriber::filter::EnvFilter;
use views::views_router;

mod adapters;
mod model;
mod state;

#[axum_macros::debug_handler]
async fn ping(State(state): State<AppState>) -> (StatusCode, &'static str) {
    let ping_result = state.connection.ping().await;
    match ping_result {
        Ok(_) => (StatusCode::OK, "OK"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database is down"),
    }
}

fn build_app(state: AppState) -> Router {
    let private_router = Router::new()
        .route("/tasks", get(get_all_tasks))
        .route("/tasks", post(create_task))
        .route("/users", get(get_all_users))
        .route("/users", post(create_user))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            authentication_middleware,
        ));
    Router::new()
        .route("/ping", get(ping))
        .nest_service("/public", static_files_service())
        .nest("/api", private_router)
        .nest("/", views_router())
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
    let state = state::new_state().await;
    let app = build_app(state.clone());
    let service = app.into_make_service_with_connect_info::<SocketAddr>();
    let target_port: u16 =
        std::env::var("PORT").map_or(8080, |port_str| port_str.parse().expect("Invalid PORT env"));
    log::info!("Trying to bind on port {}", target_port);
    let bind_addr = SocketAddr::new("0.0.0.0".parse()?, target_port);
    let listener = TcpListener::bind(bind_addr).await?;
    log::info!("Socket bound successfully, starting app");

    axum::serve(listener, service)
        .with_graceful_shutdown(async {
            let mut siggup_signal = signal(SignalKind::hangup()).unwrap();
            let mut terminate_signal = signal(SignalKind::terminate()).unwrap();
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    log::info!("SIGINT");
                }
                _ = terminate_signal.recv() => {
                    log::info!("SIGTERM")
                }
                _ = siggup_signal.recv() => {
                    log::info!("SIGHUP");
                }
            };
            log::warn!("Shutting down");
            state.connection.close().await.unwrap();
        })
        .await
        .expect("Failed to bind on port 8080");
    log::info!("End");
    Ok(())
}
