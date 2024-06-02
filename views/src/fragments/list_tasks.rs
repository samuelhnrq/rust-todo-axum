use super::error::build_error_fragment;
use axum::extract::State;
use entity::{generated::tasks, tasks::list_all};
use maud::{html, Markup};
use utils::state::HyperTarot;

#[axum_macros::debug_handler]
pub async fn fragment_controller(State(state): State<HyperTarot>) -> Markup {
    let tasks_result = list_all(&state.connection, None, None).await;
    if tasks_result.is_err() {
        return build_error_fragment("dasd");
    }
    let tasks = tasks_result.unwrap();
    list_tasks(tasks)
}

pub fn list_tasks(tasks: Vec<tasks::Model>) -> Markup {
    html! {
        table .table {
            thead {
                tr {
                    td { "Title" }
                    td { "Description" }
                }
            }
            tbody {
                @for tasks in tasks {
                    tr {
                        td { (tasks.title) }
                        td { (tasks.description) }
                    }
                }
            }
        }
    }
}
