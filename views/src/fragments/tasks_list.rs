use axum::{
  extract::{rejection::ExtensionRejection, Query, State},
  response::{IntoResponse, Redirect, Response},
  Extension,
};
use entity::{
  generated::{tasks, users},
  tasks::{delete_task, list_for_user},
};
use rinja::Template;
use serde::Deserialize;
use utils::state::HyperTarot;
use uuid::Uuid;

use super::error::ErrorTemplate;

#[derive(Deserialize)]
pub(crate) struct DeletePayload {
  task_id: Uuid,
}

#[derive(Template)]
#[template(path = "task-list.jinja.html")]
pub(crate) struct TaskListTemplate {
  tasks: Vec<tasks::Model>,
}

#[axum::debug_handler]
pub(crate) async fn delete_task_controller(
  State(state): State<HyperTarot>,
  Query(payload): Query<DeletePayload>,
) -> Redirect {
  delete_task(payload.task_id, &state.connection)
    .await
    .inspect(|_val| log::info!("deleted task {}", payload.task_id));
  Redirect::to(".")
}

#[axum::debug_handler]
pub(crate) async fn fragment_controller(
  State(state): State<HyperTarot>,
  user: Result<Extension<users::Model>, ExtensionRejection>,
) -> Response {
  let Ok(Extension(user)) = user else {
    return ErrorTemplate::new("No user".to_string()).into_response();
  };
  let tasks_result = list_for_user(&state.connection, &user, None, None).await;
  if tasks_result.is_err() {
    return ErrorTemplate::new("Could not load tasks".to_string()).into_response();
  }
  let tasks = tasks_result.unwrap();
  TaskListTemplate { tasks }.into_response()
}
