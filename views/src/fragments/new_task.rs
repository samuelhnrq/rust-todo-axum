use crate::extractors::valid_form::{InvalidForm, ValidForm};
use entity::tasks::NewTask;
use maud::{html, Markup};

#[axum_macros::debug_handler]
pub async fn fragment_new_task(form_result: Result<ValidForm<NewTask>, InvalidForm>) -> Markup {
    render_new_task(form_result.map(|form| form.0))
}

pub fn render_new_task(task_result: Result<NewTask, InvalidForm>) -> Markup {
    let mut errors: InvalidForm = InvalidForm::default();
    let task = task_result
        .inspect_err(|vals| errors = vals.clone())
        .unwrap_or_default();
    println!("here {:?} lol", errors.validation_error);
    println!("here2 {:?} lol", task);
    html! {
        form hx-post="/fragments/task" hx-target="#new-result" "hx-on:htmx:response-error"="alert('form')" {
            .mb-3 {
                label .form-label for="task-name" { "Task Name" }
                input .form-control type="text" name="title" #task-name value=(task.title);
            }
            .mb-3 {
                label .form-label for="task-description" { "Task Description" }
                input .form-control #task-description type="textbox" name="description"
                    value=(task.description.unwrap_or_default());
            }
            button .btn .btn-primary type="submit" {
                "Submit"
            }
        }
    }
}
