use crate::schema::todos;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub task_description: String,
    pub done: bool,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub title: String,
    pub task_description: Option<String>,
}

#[derive(Serialize)]
pub struct TodoListingResponse {
    pub todos: Vec<Todo>,
}
