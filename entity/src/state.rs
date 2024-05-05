use jsonwebtoken::DecodingKey;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct HyperTarot {
    pub connection: DatabaseConnection,
    pub jwk: DecodingKey,
}
