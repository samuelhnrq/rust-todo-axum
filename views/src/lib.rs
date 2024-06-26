use axum::{
    routing::{delete, get, post},
    Router,
};
use fragments::{
    list_tasks_controller, new_tasks_controller, tasks_list::delete_task_controller,
    user_fragment_controller,
};
use pages::homepage;
use utils::state::HyperTarot;

mod components;
mod errors;
mod fragments;
mod pages;

pub fn views_router() -> Router<HyperTarot> {
    let fragments_router = Router::new()
        .route("/tasks", get(list_tasks_controller))
        .route("/tasks/", get(list_tasks_controller))
        .route("/tasks", post(new_tasks_controller))
        .route("/tasks/delete", delete(delete_task_controller))
        .route("/login", get(user_fragment_controller));
    Router::new()
        .route("/", get(homepage))
        .nest("/fragments", fragments_router)
}
