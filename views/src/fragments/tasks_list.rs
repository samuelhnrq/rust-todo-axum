use crate::{
  components::spinner,
  fragments::tasks_commons::{TASK_FORM_ID_CSS, TASK_LIST_TABLE_ID, TASK_LIST_TABLE_ID_CSS},
};

use super::error::build_error_fragment;
use axum::{
  extract::{rejection::ExtensionRejection, State},
  response::Redirect,
  Extension, Form,
};
use entity::{
  generated::{tasks, users},
  tasks::{delete_task, list_for_user},
};
use maud::{html, Markup};
use serde::Deserialize;
use utils::state::HyperTarot;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct DeletePayload {
  task_id: Uuid,
}

#[axum::debug_handler]
pub(crate) async fn delete_task_controller(
  State(state): State<HyperTarot>,
  Form(payload): Form<DeletePayload>,
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
) -> Markup {
  let Ok(Extension(user)) = user else {
    return build_error_fragment("No user");
  };
  let tasks_result = list_for_user(&state.connection, &user, None, None).await;
  if tasks_result.is_err() {
    return build_error_fragment("dasd");
  }
  let tasks = tasks_result.unwrap();
  list_tasks(tasks)
}

pub(crate) fn list_tasks(tasks: Vec<tasks::Model>) -> Markup {
  html! {
      div id=(TASK_LIST_TABLE_ID) .position-relative {
          .spinner style="display: none"
              _="on htmx:beforeRequest from body set my *display to 'flex'
                 on htmx:afterRequest from body set my *display to 'none'" {
              (spinner())
          }
          table .table #all-tasks {
              thead {
                  tr {
                      td { "Title" }
                      td { "Description" }
                      td { "..." }
                  }
              }
              tbody {
                  @for tasks in tasks {
                      tr {
                          td { (tasks.title) }
                          td { (tasks.description) }
                          td {
                              button hx-delete="/fragments/tasks/delete" name="task_id" value=(tasks.id)
                                  hx-target=(TASK_LIST_TABLE_ID_CSS) .btn .btn-light .btn-sm {
                                  "🗑️"
                              }
                              button hx-post="/fragments/tasks" hx-target=(TASK_FORM_ID_CSS) name="edit_target"
                                  value=(tasks.id) .btn .btn-light .btn-sm {
                                  "✏️"
                              }
                          }
                      }
                  }
              }
          }
          button .btn .btn-secondary #refresh-tasks hx-get="./fragments/tasks"
              hx-target=(TASK_LIST_TABLE_ID_CSS) {
              "Refresh list"
          }
      }
  }
}
