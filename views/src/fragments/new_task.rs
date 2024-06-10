use std::fmt::Display;

use axum::{extract::State, Extension, Form};
use entity::{generated::users, tasks::NewTask};
use maud::{html, Markup};
use serde::Deserialize;
use serde_valid::{
    validation::{Errors, PropertyErrorsMap},
    Validate,
};
use utils::state::HyperTarot;
use uuid::Uuid;

#[derive(Deserialize, Default, Debug, Clone, Validate)]
pub struct Payload {
    #[validate(min_length = 3)]
    pub title: Option<String>,
    pub owner: Option<Uuid>,
    pub description: Option<String>,
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

impl TryFrom<Payload> for NewTask {
    type Error = PayloadConversionError;

    fn try_from(value: Payload) -> Result<Self, Self::Error> {
        if value.owner.is_none() || value.title.is_none() {
            Err(PayloadConversionError {})
        } else {
            Ok(NewTask {
                description: value.description,
                owner: value.owner.unwrap(),
                title: value.title.unwrap(),
            })
        }
    }
}

pub fn text_field<T: Into<String>>(
    field: &'static str,
    value: Option<T>,
    errors: Option<&Errors>,
) -> Markup {
    let id = "form-field-".to_owned() + field;
    let id_desc = "form-field-".to_owned() + field + "-desc";
    html! {
        .form-group {
            label .form-label for=(id) { "Task Description" }
            input .form-control id=(id) type="textbox" name=(field) aria-describedby=(id_desc)
                value=[value.map(Into::into)];
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
    let in_task = if let Some(task) = task_result {
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
            let res = entity::tasks::new_task(new_task, &state.connection).await;
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
        in_task
    };
    html! {
        @if let Some(created) = maybe_new_task {
            div _="init trigger click on #refresh-tasks" {
                "Task created successfully " (created.id.to_string())
            }
        }
        form #new-result hx-post="/fragments/tasks" hx-target="#new-result" "hx-on:htmx:response-error"="alert('form')" {
            // TODO: wire from request handlers userdata extension into here as parameter
            input type="hidden" name="owner" value=[user.map(|u| u.id)];
            .mb-3 {
                (text_field("title", task.title, error_map.get("title")))
            }
            .mb-3 {
                (text_field("description", task.description, error_map.get("description")))
            }
            button .btn .btn-primary type="submit" _="on click add .disabled" {
                "Submit"
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
