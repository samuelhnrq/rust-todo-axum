use axum::{
  extract::{rejection::ExtensionRejection, State},
  Extension,
};
use entity::{generated::users, tasks::list_for_user};
use maud::{html, Markup};
use utils::state::HyperTarot;

use crate::{
  components::scaffolding,
  fragments::{list_tasks, new_task},
};

#[axum::debug_handler]
pub async fn homepage(
  State(state): State<HyperTarot>,
  usr: Result<Extension<users::Model>, ExtensionRejection>,
) -> Markup {
  let tasks_result = if let Ok(Extension(user)) = usr.as_ref() {
    list_for_user(&state.connection, user, None, None)
      .await
      .unwrap_or_default()
  } else {
    vec![]
  };
  let body = html! {
    h1.display-2 { "Welcome to Hyper-Tarot!" }
    @if usr.is_ok() {
      h1 { "Available tasks" }
      (list_tasks(tasks_result))
      (new_task(state, None, usr.map(|Extension(u)| u).ok()).await)
    } @else {
      h2 {
        "Please login!"
      }
      a hx-boost="false" .btn .btn-primary
          _="on mutation of anything from <nav.navbar/> set @href to #login-anchor@href
            on load set @href to #login-anchor@href" {
        "Login"
      }
    }
  };
  scaffolding("Hello World", &body)
}
