use jsonwebtoken::jwk::Jwk;
use sea_orm::DatabaseConnection;

use crate::adapters::{authentication::fetch_remote_jwk, database::connect_database};

#[derive(Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
    pub jwk: Jwk,
}

impl AppState {
    pub async fn new() -> AppState {
        let connection = connect_database().await;
        let jwk = fetch_remote_jwk().await;
        AppState { connection, jwk }
    }
}
