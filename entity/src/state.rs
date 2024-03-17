use jsonwebtoken::DecodingKey;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
    pub jwk: DecodingKey,
}
