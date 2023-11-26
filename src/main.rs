use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use diesel::prelude::*;
use diesel::{insert_into, r2d2::ConnectionManager, PgConnection, QueryDsl, SelectableHelper};
use r2d2::{ManageConnection, Pool, PooledConnection};
use state::AppState;
use tracing_subscriber::filter::EnvFilter;

use crate::model::{NewTodo, Todo, TodoListingResponse};

mod migrations;
mod model;
mod schema;
mod state;

fn get_connetion<M: ManageConnection>(
    conn: &Pool<M>,
) -> Result<PooledConnection<M>, (StatusCode, String)> {
    return conn.get_timeout(Duration::from_millis(500)).map_err(|_| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "Not able to connect".into(),
        )
    });
}

#[axum_macros::debug_handler]
async fn list_all_todos(
    State(state): State<AppState>,
) -> Result<Json<TodoListingResponse>, (StatusCode, String)> {
    use self::schema::todos::dsl::*;
    log::info!("listing all todos");
    let mut con = state
        .conn
        .get_timeout(Duration::from_millis(500))
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Not able to connect".into(),
            )
        })?;
    log::info!("connected to the database");
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
    log::info!("query ran successfully, serializing");
    return Ok(Json(TodoListingResponse { todos: tasks }));
}

#[axum_macros::debug_handler]
async fn create_new_todo(
    State(state): State<AppState>,
    Json(req_body): Json<NewTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    use self::schema::todos::dsl::*;
    log::info!("listing all todos");
    let mut con = get_connetion(&state.conn)?;
    log::info!("connected to the database");
    let task = insert_into(todos)
        .values(&req_body)
        .get_result::<Todo>(&mut con)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert".into()))?;
    log::info!("query ran successfully, serializing");
    return Ok(Json(task));
}

type PgPool = Pool<ConnectionManager<PgConnection>>;

fn build_app(pool: PgPool) -> Router {
    // run it with hyper on localhost:8080
    let trace = TraceLayer::new_for_http();
    return Router::new()
        .route("/todos", get(list_all_todos))
        .route("/todos", post(create_new_todo))
        .layer(trace)
        .with_state(AppState { conn: pool });
    // return app;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::from_default_env();
    tracing_subscriber::fmt().with_env_filter(filter).init();
    // build our application with a single route
    log::info!("Initializing, connecting to the database");
    let db_url = std::env::var("DATABASE_URL").expect("Missing env variable DATABASE_URL");
    let mgr = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(mgr)?;
    log::info!("Connected successfully, running migrations");
    migrations::run_migrations(&mut pool.get()?).unwrap();
    log::info!("Migrations OK, Will serve on 8080");
    let app = build_app(pool);
    let service = app.into_make_service();
    let target_port: u16 =
        std::env::var("PORT").map_or(8080, |port_str| port_str.parse().expect("Invalid PORT env"));
    let bind_addr = SocketAddr::new("0.0.0.0".parse()?, target_port);
    axum::Server::bind(&bind_addr)
        .serve(service)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await
        .expect("Failed to bind on port 8080");
    log::info!("End");
    Ok(())
}
