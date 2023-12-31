use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use entity::tasks::{list_all_tasks, new_task, NewTask, Task};

#[axum_macros::debug_handler]
pub async fn get_all_tasks(
    State(state): State<AppState>,
) -> Result<Json<Vec<Task>>, (StatusCode, &'static str)> {
    let tasks = list_all_tasks(&state.connection).await.map_err(|err| {
        log::error!("Error listing tasks:\n{}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to query");
    })?;
    return Ok(Json(tasks));
}

#[axum_macros::debug_handler]
pub async fn create_task(
    State(state): State<AppState>,
    Json(body): Json<NewTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let new_task = new_task(body, &state.connection).await.map_err(|err| {
        log::error!("Failed to create task! err: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create".into())
    })?;
    return Ok(Json(new_task));
}
