use std::error::Error;
use std::time::Duration;

use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use diesel::prelude::*;
use diesel::{r2d2, PgConnection, QueryDsl, SelectableHelper};
use state::AppState;

use crate::model::{Todo, TodoListingResponse};

mod model;
mod schema;
mod state;

#[axum_macros::debug_handler]
async fn list_all_todos(
    State(state): State<AppState>,
) -> Result<Json<TodoListingResponse>, (StatusCode, String)> {
    use self::schema::todos::dsl::*;
    println!("listing all todos");
    let mut con = state
        .conn
        .get_timeout(Duration::from_millis(500))
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Not able to connect".into(),
            )
        })?;
    println!("connected to the database");
    let tasks: Vec<Todo> = todos
        .limit(100)
        .select(Todo::as_select())
        .load(&mut con)
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "could not query stuff".into(),
            )
        })?;
    println!("query ran successfully, serializing");
    return Ok(Json(TodoListingResponse { todos: tasks }));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // build our application with a single route
    let mgr = r2d2::ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL").expect("Missing database URL"),
    );
    let pool = r2d2::Pool::builder().build(mgr)?;
    println!("Will serve on 3000");
    // run it with hyper on localhost:3000
    let app = Router::new().route("/todos", get(list_all_todos));
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.with_state(AppState { conn: pool }).into_make_service())
        .await
        .unwrap_or_default();
    println!("unreachable");
    Ok(())
}
