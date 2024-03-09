use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use entity::users::{list_all_users, new_user, NewUser, User};

#[axum_macros::debug_handler]
pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, &'static str)> {
    let users = list_all_users(&state.connection, None, None)
        .await
        .map_err(|err| {
            log::error!("Error listing users:\n{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to query")
        })?;
    Ok(Json(users))
}

#[axum_macros::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, &'static str)> {
    let new_user = new_user(body, &state.connection).await.map_err(|err| {
        log::error!("Failed to create user! err:\n{}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create")
    })?;
    Ok(Json(new_user))
}
