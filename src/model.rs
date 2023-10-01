use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub todo_id: i32,
    pub todo_title: String,
    pub todo_description: String,
    pub todo_done: bool,
}

#[derive(Serialize)]
pub struct TodoListingResponse {
    pub todos: Vec<Todo>,
}
