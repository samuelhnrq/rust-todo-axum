use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;

use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use diesel::prelude::*;
use diesel::{r2d2, PgConnection, QueryDsl, SelectableHelper};
use state::AppState;

use crate::model::{Todo, TodoListingResponse};

mod migrations;
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
    println!("Initializing, connecting to the database");
    let mgr = r2d2::ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL").expect("Missing env variable DATABASE_URL"),
    );
    let pool = r2d2::Pool::builder().build(mgr)?;
    println!("Connected successfully, running migrations");
    migrations::run_migrations(&mut pool.get()?).unwrap();
    println!("Will serve on 8080");
    // run it with hyper on localhost:8080
    let app = Router::new()
        .route("/todos", get(list_all_todos))
        .with_state(AppState { conn: pool });
    let target_port = std::env::var("PORT").map_or(8080, |port_str| {
        port_str.parse::<u16>().expect("Invalid PORT env")
    });
    let bind_addr = SocketAddr::new("0.0.0.0".parse()?, target_port);
    axum::Server::bind(&bind_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await
        .expect("Failed to bind on port 8080");
    println!("end");
    Ok(())
}
