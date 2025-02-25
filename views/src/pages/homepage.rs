use crate::filters;
use crate::fragments::CreateTaskPayload;
use axum::{
  extract::{rejection::ExtensionRejection, State},
  Extension,
};
use entity::{
  generated::{tasks, users},
  tasks::list_for_user,
};
use rinja::Template;
use utils::state::HyperTarot;

#[derive(Template, Default)]
#[template(path = "pages/index.jinja.html")]
pub(crate) struct HomeTemplate {
  user: Option<users::Model>,
  tasks: Vec<tasks::Model>,
  task: CreateTaskPayload,
}

#[axum::debug_handler]
pub(crate) async fn homepage(
  State(state): State<HyperTarot>,
  user: Result<Extension<users::Model>, ExtensionRejection>,
) -> HomeTemplate {
  let tasks_result = if let Ok(Extension(user)) = user.as_ref() {
    list_for_user(&state.connection, user, None, None)
      .await
      .unwrap_or_default()
  } else {
    vec![]
  };
  HomeTemplate {
    tasks: tasks_result,
    task: CreateTaskPayload::default(),
    user: user.ok().map(|Extension(u)| u),
  }
}
