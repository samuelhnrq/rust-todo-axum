use axum::{
    extract::{rejection::FormRejection, State},
    Form,
};
use entity::tasks::NewTask;
use maud::{html, Markup};
use serde_valid::{validation::Errors, validation::PropertyErrorsMap, Validate};
use utils::state::HyperTarot;

#[axum_macros::debug_handler]
pub async fn fragment_controller(
    State(state): State<HyperTarot>,
    form_result: Result<Form<NewTask>, FormRejection>,
) -> Markup {
    new_task(state, Some(form_result)).await
}

pub fn text_field<T: Into<String>>(
    field: &'static str,
    value: T,
    errors: Option<&Errors>,
) -> Markup {
    let id = "form-field-".to_owned() + field;
    let id_desc = "form-field-".to_owned() + field + "-desc";
    html! {
        .form-group {
            label .form-label for=(id) { "Task Description" }
            input .form-control id=(id) type="textbox" name=(field) aria-describedby=(id_desc)
                value=(value.into());
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
    task_result: Option<Result<Form<NewTask>, FormRejection>>,
) -> Markup {
    let form_ok = matches!(task_result, Some(Ok(Form(_))));
    let task = if let Some(Ok(Form(task))) = task_result {
        task
    } else {
        NewTask::default()
    };
    let error_map = if let Err(Errors::Object(v)) = task.validate() {
        v.properties
    } else {
        PropertyErrorsMap::new()
    };
    let uploaded = if form_ok && error_map.is_empty() {
        let res = entity::tasks::new_task(task.clone(), &state.connection).await;
        res.is_ok()
    } else {
        false
    };
    html! {
        form #new-result hx-post="/fragments/task" hx-target="#new-result" "hx-on:htmx:response-error"="alert('form')" {
            // TODO: wire from request handlers userdata extension into here as parameter
            input type="hidden" name="owner" value=(task.owner)
                _="init wait 1s then set my value to Clerk.session.user.id";
            .mb-3 {
                (text_field("title", task.title, error_map.get("title")))
            }
            .mb-3 {
                (text_field("description", task.description.unwrap_or_default(), error_map.get("description")))
            }
            div {
                "I have been " (uploaded.to_string())
            }
            button .btn .btn-primary type="submit" _="on click add .disabled" {
                "Submit"
            }
        }
    }
}
