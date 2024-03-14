use jsonwebtoken::jwk::Jwk;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
    pub jwk: Jwk,
}
