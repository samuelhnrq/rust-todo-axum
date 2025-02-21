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
use utils::state::HyperTarot; // bring trait in scope

#[derive(Template, Default)] // this will generate the code...
#[template(path = "pages/index.jinja.html")] // using the template in this path, relative
pub(crate) struct HomeTemplate {
  usr: Option<users::Model>, // the field name should match the variable name
  tasks: Vec<tasks::Model>,  // the field name should match the variable name
  task: CreateTaskPayload,
}

#[axum::debug_handler]
pub(crate) async fn homepage(
  State(state): State<HyperTarot>,
  usr: Result<Extension<users::Model>, ExtensionRejection>,
) -> HomeTemplate {
  let tasks_result = if let Ok(Extension(user)) = usr.as_ref() {
    list_for_user(&state.connection, user, None, None)
      .await
      .unwrap_or_default()
  } else {
    vec![]
  };
  HomeTemplate {
    tasks: tasks_result,
    task: CreateTaskPayload::default(),
    usr: usr.ok().map(|Extension(u)| u),
  }
}
