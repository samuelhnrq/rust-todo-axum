use std::error::Error;

use crate::errors::extract_serde_error_list;
use axum::{extract::rejection::FormRejection, Form};
use entity::tasks::NewTask;
use maud::{html, Markup};

#[axum_macros::debug_handler]
pub async fn fragment_new_task(form_result: Result<Form<NewTask>, FormRejection>) -> Markup {
    println!("the new task handler");
    render_new_task(form_result.map(|form| form.0).map_err(test))
}

fn test(err: FormRejection) -> Vec<i32> {
    println!("were here");
    extract_serde_error_list(err.source().unwrap())
}

pub fn render_new_task(task_result: Result<NewTask, Vec<i32>>) -> Markup {
    let mut errors = vec![];
    let task = match task_result {
        Ok(vala) => vala,
        Err(vals) => {
            errors = vals;
            NewTask::default()
        }
    };
    println!("here {} lol", errors.len());
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
