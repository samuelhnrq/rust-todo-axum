use axum::{
    routing::{get, post},
    Router,
};
use entity::HyperTarot;
use fragments::{list_tasks_controller, new_tasks_controller};
use pages::homepage;

mod components;
mod errors;
mod fragments;
mod pages;

pub fn views_router() -> Router<HyperTarot> {
    let fragments_router = Router::new()
        .route("/task", get(list_tasks_controller))
        .route("/task", post(new_tasks_controller));
    Router::new()
        .route("/", get(homepage))
        .nest("/fragments", fragments_router)
}
