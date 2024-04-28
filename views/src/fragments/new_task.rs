use axum::{
    extract::{rejection::FormRejection, State},
    Form,
};
use entity::{tasks::NewTask, AppState};
use maud::{html, Markup};
use serde_valid::{validation::Errors, validation::PropertyErrorsMap, Validate};

#[axum_macros::debug_handler]
pub async fn fragment_new_task(
    State(state): State<AppState>,
    form_result: Result<Form<NewTask>, FormRejection>,
) -> Markup {
    render_new_task(state, Some(form_result)).await
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
            @match errors {
                Some(Errors::NewType(val)) => @for err in val {
                    .form-text .text-danger id=(id_desc) { (err.to_string()) }
                },
                _ => ""
            }
        }
    }
}

pub async fn render_new_task(
    state: AppState,
    task_result: Option<Result<Form<NewTask>, FormRejection>>,
) -> Markup {
    let is_missing = task_result.is_none();
    let form_ok = matches!(task_result, Some(Ok(Form(_))));
    let task = match task_result {
        Some(Ok(Form(task))) => task,
        _ => NewTask::default(),
    };
    let error_map = if is_missing {
        PropertyErrorsMap::new()
    } else {
        match task.validate() {
            Err(Errors::Object(v)) => v.properties,
            _ => PropertyErrorsMap::new(),
        }
    };
    let uploaded = if form_ok && error_map.is_empty() {
        let res = entity::tasks::new_task(task.clone(), &state.connection).await;
        log::info!("wow {:?}", res.is_ok());
        res.is_ok()
    } else {
        false
    };
    html! {
        form #new-result hx-post="/fragments/task" hx-target="#new-result" "hx-on:htmx:response-error"="alert('form')" {
            // TODO: wire from request handlers userdata extension into here as parameter
            input type="hidden" name="owner" value=(task.owner);
            .mb-3 {
                (text_field("title", task.title, error_map.get("title")))
            }
            .mb-3 {
                (text_field("description", task.description.unwrap_or_default(), error_map.get("description")))
            }
            div {
                "I have been " (uploaded.to_string())
            }
            button .btn .btn-primary type="submit" {
                "Submit"
            }
        }
    }
}
