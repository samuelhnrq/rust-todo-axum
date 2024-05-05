use axum::extract::State;
use entity::{tasks::list_all, HyperTarot};
use maud::{html, Markup};

use crate::{
    components::scaffolding,
    fragments::{list_tasks, new_task},
};

#[axum_macros::debug_handler]
pub async fn homepage(State(state): State<HyperTarot>) -> Markup {
    let tasks_result = list_all(&state.connection, None, None).await.unwrap();
    let body = html! {
        h1.display-2 { "Hello HTMX!" }
        h1 { "Available tasks" }
        #test { (list_tasks(tasks_result)) }
        button .btn .btn-secondary #refresh-tasks hx-get="./fragments/tasks" hx-target="#test" {
            "Refresh list"
        }
        #new-task-form { (new_task(state, None).await) }
    };
    scaffolding("Hello World", &body)
}
