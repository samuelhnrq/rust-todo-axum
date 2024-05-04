use super::error::build_error_fragment;
use axum::extract::State;
use entity::{tasks::list_all_tasks, AppState, Task};
use maud::{html, Markup};

#[axum_macros::debug_handler]
pub async fn tasks_fragment(State(state): State<AppState>) -> Markup {
    let tasks_result = list_all_tasks(&state.connection, None, None).await;
    if tasks_result.is_err() {
        return build_error_fragment("dasd");
    }
    let tasks = tasks_result.unwrap();
    render_task_list(tasks)
}

pub fn render_task_list(tasks: Vec<Task>) -> Markup {
    html! {
        table .table {
            thead {
                tr {
                    td { "Title" }
                    td { "Description" }
                }
            }
            tbody {
                @for task in tasks {
                    tr {
                        td { (task.title) }
                        td { (task.description) }
                    }
                }
            }
        }
    }
}
