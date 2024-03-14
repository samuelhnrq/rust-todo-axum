use axum::{routing::get, Router};
use entity::AppState;
use fragments::tasks_fragment;
use homepage::homepage;

mod fragments;
mod homepage;

pub fn views_router() -> Router<AppState> {
    Router::new()
        .route("/", get(homepage))
        .route("/fragments/wow", get(tasks_fragment))
}
