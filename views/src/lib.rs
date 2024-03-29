use axum::{
    routing::{get, post},
    Router,
};
use entity::AppState;
use fragments::{fragment_new_task, tasks_fragment};
use pages::homepage;

mod components;
mod fragments;
mod pages;

pub fn views_router() -> Router<AppState> {
    let fragments_router = Router::new()
        .route("/tasks", get(tasks_fragment))
        .route("/tasks", post(fragment_new_task));
    Router::new()
        .route("/", get(homepage))
        .nest("/fragments", fragments_router)
}
