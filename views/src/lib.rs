use axum::{
  routing::{delete, get, post},
  Router,
};
use fragments::{
  delete_task_controller, list_tasks_controller, new_tasks_controller, user_fragment_controller,
};
use pages::homepage;
use utils::state::HyperTarot;

mod errors;
mod filters;
mod fragments;
mod pages;

pub(crate) const TASK_LIST_TABLE_ID: &str = "tasks_list_table";
pub(crate) const TASK_FORM_ID: &str = "tasks_form";

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
