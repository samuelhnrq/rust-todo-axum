use axum::extract::State;
use entity::{
    tasks::{list_all_tasks, NewTask},
    AppState,
};
use maud::{html, Markup};

use crate::{
    components::scaffolding,
    fragments::{render_new_task, render_task_list},
};

#[axum_macros::debug_handler]
pub async fn homepage(State(state): State<AppState>) -> Markup {
    let tasks_result = list_all_tasks(&state.connection, None, None).await.unwrap();
    let body = html! {
        h1.display-2 { "Hello HTMX!" }
        h1 { "Available tasks" }
        #test { (render_task_list(tasks_result)) }
        button .btn .btn-secondary #refresh-tasks hx-get="./fragments/tasks" hx-target="#test" {
            "Refresh list"
        }
        #new-result;
        #new-task-form { (render_new_task(NewTask::default())) }
    };
    scaffolding("Hello World", body)
}
