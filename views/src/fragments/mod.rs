mod error;
pub(crate) mod list_tasks;
mod new_task;
mod user_auth;

pub(crate) use list_tasks::{fragment_controller as list_tasks_controller, list_tasks};
pub(crate) use new_task::{fragment_controller as new_tasks_controller, new_task};
pub(crate) use user_auth::fragment_controller as user_fragment_controller;
