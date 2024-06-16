mod error;
mod tasks_commons;
mod tasks_create;
pub(crate) mod tasks_list;
mod user_auth;

pub(crate) use tasks_create::{fragment_controller as new_tasks_controller, new_task};
pub(crate) use tasks_list::{fragment_controller as list_tasks_controller, list_tasks};
pub(crate) use user_auth::fragment_controller as user_fragment_controller;
