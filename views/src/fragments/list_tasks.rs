use super::error::build_error_fragment;
use axum::{extract::State, response::Redirect, Form};
use entity::{
    generated::tasks,
    tasks::{delete_task, list_all},
};
use maud::{html, Markup};
use serde::Deserialize;
use utils::state::HyperTarot;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct DeletePayload {
    task_id: Uuid,
}

#[axum_macros::debug_handler]
pub(crate) async fn delete_task_controller(
    State(state): State<HyperTarot>,
    Form(payload): Form<DeletePayload>,
) -> Redirect {
    delete_task(payload.task_id, &state.connection)
        .await
        .inspect(|_val| log::info!("deleted task {}", payload.task_id));
    Redirect::to(".")
}

#[axum_macros::debug_handler]
pub(crate) async fn fragment_controller(State(state): State<HyperTarot>) -> Markup {
    let tasks_result = list_all(&state.connection, None, None).await;
    if tasks_result.is_err() {
        return build_error_fragment("dasd");
    }
    let tasks = tasks_result.unwrap();
    list_tasks(tasks)
}

pub(crate) fn list_tasks(tasks: Vec<tasks::Model>) -> Markup {
    html! {
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
                                hx-target="#all-tasks" .btn .btn-light .btn-sm {
                                "🗑️"
                            }
                            button hx-get="/fragments/tasks" hx-target="#test" name="task_id" value=(tasks.id)
                                .btn .btn-light .btn-sm {
                                "✏️"
                            }
                        }
                    }
                }
            }
        }
    }
}
