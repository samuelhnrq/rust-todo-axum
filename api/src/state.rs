use entity::AppState;

use crate::adapters::{authentication::fetch_remote_jwk, database::connect_database};

pub async fn new_state() -> AppState {
    let connection = connect_database().await;
    let jwk = fetch_remote_jwk().await;
    AppState { connection, jwk }
}
