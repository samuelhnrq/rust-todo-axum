use std::error::Error;
use std::net::SocketAddr;

use crate::adapters::{
    controllers::{tasks, users},
    static_files::build_service,
};
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tower_http::trace::TraceLayer;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use utils::authentication::{
    handle_oauth_redirect, required_login_middleware, user_data_extension, REDIRECT_PATH,
};
use utils::state::HyperTarot;
use views::views_router;

mod adapters;
mod state;

#[axum_macros::debug_handler]
async fn ping(State(state): State<HyperTarot>) -> (StatusCode, &'static str) {
    let ping_result = state.connection.ping().await;
    match ping_result {
        Ok(()) => (StatusCode::OK, "OK"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database is down"),
    }
}

fn build_app(state: HyperTarot) -> Router {
    let private_router = Router::new()
        .route("/tasks", get(tasks::get_all))
        .route("/tasks", post(tasks::create))
        .route("/users", get(users::get_all))
        .route("/users", post(users::create))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            required_login_middleware,
        ));
    Router::new()
        .nest("/api", private_router)
        .nest("/", views_router())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            user_data_extension,
        ))
        .route(REDIRECT_PATH, get(handle_oauth_redirect))
        .nest_service("/public", build_service())
        .route("/ping", get(ping))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn Error>> {
    let env_log_config = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(env_log_config)
        .init();
    log::info!("Initializing, connecting to the database");
    let state = state::create().await;
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
                    log::info!("SIGTERM");
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
    log::info!("Good bye");
    Ok(())
}
