use std::fmt::Display;

use axum::{extract::State, Extension, Form};
use entity::{generated::users, tasks::UpsertTask};
use maud::{html, Markup};
use serde::Deserialize;
use serde_valid::{
    validation::{Errors, PropertyErrorsMap},
    Validate,
};
use utils::state::HyperTarot;
use uuid::Uuid;

use crate::{
    components::spinner,
    fragments::tasks_commons::{TASK_FORM_ID, TASK_FORM_ID_CSS},
};

#[derive(Deserialize, Default, Debug, Clone, Validate)]
pub struct Payload {
    #[validate(min_length = 3)]
    pub title: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>,
    pub edit_target: Option<String>,
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

impl TryFrom<Payload> for UpsertTask {
    type Error = PayloadConversionError;

    fn try_from(value: Payload) -> Result<Self, Self::Error> {
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

pub fn text_field<T: Into<String>>(
    field: &'static str,
    value: Option<T>,
    pattern: Option<&str>,
    errors: Option<&Errors>,
) -> Markup {
    let id = "form-field-".to_string() + field;
    let id_desc = "form-field-".to_string() + field + "-desc";
    html! {
        .form-group {
            label .form-label for=(id) .text-capitalize { (field) }
            input .form-control id=(id) type="textbox" name=(field) aria-describedby=(id_desc)
                value=[value.map(Into::into)] pattern=[pattern];
            @if let Some(Errors::NewType(val)) = errors {
                @for err in val {
                    .form-text .text-danger id=(id_desc) { (err.to_string()) }
                }
            }
        }
    }
}

pub async fn new_task(
    state: HyperTarot,
    task_result: Option<Payload>,
    user: Option<users::Model>,
) -> Markup {
    let form_ok = task_result.is_some();
    let mut in_task = if let Some(task) = task_result {
        log::debug!("parsed form");
        task
    } else {
        log::info!("Found no task");
        Payload::default()
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
            res.inspect_err(|err| log::info!("Failed to insert the task: {:?}", err))
                .inspect(|_usr| log::info!("inserted new task successfully"))
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
        Payload::default()
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
    html! {
        div id=(TASK_FORM_ID) .position-relative {
            h2 .my-3 {
                @if task.edit_target.as_ref().map_or(true, String::is_empty) {
                    "Create task"
                } @else {
                    "Edit Task"
                }
            }
            .spinner  style="display: none"
                _="on htmx:beforeRequest from body set my *display to 'flex'
                   on htmx:afterRequest from body set my *display to 'none'"  {
                (spinner())
            }
            form #task-edit-form hx-post="/fragments/tasks" _="on htmx:afterRequest wait 0.1s then send click to #refresh-tasks"
                hx-target=(TASK_FORM_ID_CSS) .my-3 {
                // TODO: wire from request handlers userdata extension into here as parameter
                input type="hidden" name="owner" value=[user.map(|u| u.id)];
                input type="hidden" name="edit_target" value=[task.edit_target];
                .mb-3 {
                    (text_field("title", task.title, Some("\\w{,3}"), error_map.get("title")))
                }
                .mb-3 {
                    (text_field("description", task.description, None, error_map.get("description")))
                }
                input .btn .btn-primary type="submit";
            }
        }
    }
}

#[axum_macros::debug_handler]
pub async fn fragment_controller(
    State(state): State<HyperTarot>,
    maybe_usr: Option<Extension<users::Model>>,
    form_result: Option<Form<Payload>>,
) -> Markup {
    new_task(
        state,
        form_result.map(|Form(x)| x),
        maybe_usr.map(|Extension(x)| x),
    )
    .await
}
