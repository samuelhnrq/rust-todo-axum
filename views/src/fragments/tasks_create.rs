use std::fmt::Display;

use crate::filters;
use axum::{
  extract::{
    rejection::{ExtensionRejection, FormRejection},
    State,
  },
  Extension, Form,
};
use entity::{generated::users, tasks::UpsertTask};
use rinja::Template;
use serde::Deserialize;
use serde_valid::{
  validation::{Errors, PropertyErrorsMap},
  Validate,
};
use utils::state::HyperTarot;
use uuid::Uuid;

#[derive(Deserialize, Default, Debug, Clone, Validate)]
pub struct CreateTaskPayload {
  #[validate(min_length = 3)]
  pub title: Option<String>,
  pub owner: Option<String>,
  pub description: Option<String>,
  pub edit_target: Option<String>,
}

#[derive(Template)] // this will generate the code...
#[template(path = "task-form.jinja.html")] // using the template in this path, relative
pub(crate) struct TaskFormTemplate {
  user: Option<users::Model>,
  task: CreateTaskPayload,
}

#[derive(Debug)]
pub struct PayloadConversionError {}

impl Display for PayloadConversionError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("failed to convert").ok();
    Ok(())
  }
}

impl std::error::Error for PayloadConversionError {}

impl TryFrom<CreateTaskPayload> for UpsertTask {
  type Error = PayloadConversionError;

  fn try_from(value: CreateTaskPayload) -> Result<Self, Self::Error> {
    let err = PayloadConversionError {};
    if value.owner.is_none() || value.title.is_none() {
      Err(err)
    } else {
      Ok(UpsertTask {
        edit_target: value
          .edit_target
          .and_then(|t| Uuid::parse_str(t.as_str()).ok()),
        description: value.description,
        owner: value
          .owner
          .and_then(|o| Uuid::parse_str(o.as_str()).ok())
          .ok_or(err)?,
        title: value.title.unwrap(),
      })
    }
  }
}

pub async fn new_task(
  state: HyperTarot,
  task_result: Option<CreateTaskPayload>,
  user: Option<users::Model>,
) -> TaskFormTemplate {
  let form_ok = task_result.is_some();
  let mut in_task = if let Some(task) = task_result {
    log::debug!("parsed form");
    task
  } else {
    log::info!("Found no task");
    CreateTaskPayload::default()
  };
  let error_map = if form_ok {
    if let Err(Errors::Object(v)) = in_task.validate() {
      v.properties
    } else {
      PropertyErrorsMap::new()
    }
  } else {
    PropertyErrorsMap::new()
  };
  let maybe_new_task = if error_map.is_empty() {
    if let Ok(new_task) = in_task.clone().try_into() {
      log::info!("Task form is ok, trying to insert");
      let res = entity::tasks::upsert_task(new_task, &state.connection).await;
      res
        .inspect_err(|err| log::info!("Failed to insert the task: {:?}", err))
        .inspect(|_| log::info!("inserted new task successfully"))
        .ok()
    } else {
      log::info!("did not manage to convert to task insert");
      None
    }
  } else {
    None
  };
  let uploaded = maybe_new_task.is_some();
  let task = if uploaded {
    CreateTaskPayload::default()
  } else {
    if let Some(ref target) = in_task.edit_target {
      log::debug!("looking for task to edit");
      let target_id = Uuid::parse_str(target).unwrap_or_default();
      let maybe_target = entity::tasks::get_by_id(&state.connection, &target_id).await;
      if let Some(loaded) = maybe_target {
        log::debug!("loaded task for editing");
        in_task.owner = Some(loaded.owner_id.to_string());
        in_task.title = Some(loaded.title);
        in_task.description = Some(loaded.description);
      }
    }
    in_task
  };
  TaskFormTemplate { user, task }
}

#[axum::debug_handler]
pub(crate) async fn fragment_controller(
  State(state): State<HyperTarot>,
  maybe_user: Result<Extension<users::Model>, ExtensionRejection>,
  form_result: Result<Form<CreateTaskPayload>, FormRejection>,
) -> TaskFormTemplate {
  new_task(
    state,
    form_result.map(|Form(x)| x).ok(),
    maybe_user.map(|Extension(x)| x).ok(),
  )
  .await
}
