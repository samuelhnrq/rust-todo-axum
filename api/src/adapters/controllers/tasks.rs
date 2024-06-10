use axum::{extract::State, http::StatusCode, Json};
use entity::{generated::tasks, tasks::list_all};
use utils::state::HyperTarot;

#[axum_macros::debug_handler]
pub async fn get_all(
    State(state): State<HyperTarot>,
) -> Result<Json<Vec<tasks::Model>>, (StatusCode, &'static str)> {
    let tasks = list_all(&state.connection, None, None)
        .await
        .map_err(|err| {
            log::error!("Error listing tasks:\n{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to query")
        })?;
    Ok(Json(tasks))
}

// #[axum_macros::debug_handler]
// pub async fn create(
//     State(state): State<HyperTarot>,
//     Form(body): Form<NewTask>,
// ) -> Result<Json<tasks::Model>, (StatusCode, &'static str)> {
//     let new_task = new_task(body, &state.connection).await.map_err(|err| {
//         log::error!("Failed to create tasks! err:\n{}", err);
//         (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create")
//     })?;
//     Ok(Json(new_task))
// }
