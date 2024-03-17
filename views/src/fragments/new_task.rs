use axum::Form;
use entity::tasks::NewTask;
use maud::{html, Markup};

#[axum_macros::debug_handler]
pub async fn fragment_new_task(Form(task): Form<NewTask>) -> Markup {
    render_new_task(task)
}

pub fn render_new_task(task: NewTask) -> Markup {
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
