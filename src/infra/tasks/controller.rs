use crate::{
    state::AppState,
    tasks_repository::{list_all_tasks, Task},
};
use axum::{extract::State, http::StatusCode, Json};

#[axum_macros::debug_handler]
pub async fn get_all_tasks(
    State(state): State<AppState>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let tasks = list_all_tasks(&state.connection)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to query".into()))?;
    return Ok(Json(tasks));
}
