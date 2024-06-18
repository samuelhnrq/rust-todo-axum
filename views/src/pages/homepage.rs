use axum::{extract::State, Extension};
use entity::{generated::users, tasks::list_for_user};
use maud::{html, Markup};
use utils::state::HyperTarot;

use crate::{
    components::scaffolding,
    fragments::{list_tasks, new_task},
};

#[axum_macros::debug_handler]
pub async fn homepage(
    State(state): State<HyperTarot>,
    usr: Option<Extension<users::Model>>,
) -> Markup {
    log::info!("rendering homepage");
    let tasks_result = if let Some(user) = usr.as_ref() {
        list_for_user(&state.connection, user, None, None)
            .await
            .unwrap_or_default()
    } else {
        vec![]
    };
    let body = html! {
        h1.display-2 { "Welcome to Hyper-Tarot!" }
        @if usr.is_some() {
            h1 { "Available tasks" }
            (list_tasks(tasks_result))
            (new_task(state, None, usr.map(|Extension(u)| u)).await)
        } @else {
            h2 {
                "Please login!"
            }
            a hx-boost="false" .btn .btn-primary
                    _="on mutation of anything from closest <div/> to #login-anchor set @href to #login-anchor@href" {
                "Login"
            }
        }
    };
    scaffolding("Hello World", &body)
}
