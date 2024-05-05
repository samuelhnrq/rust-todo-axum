use jsonwebtoken::DecodingKey;
use reqwest::Client;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct HyperTarot {
    pub connection: DatabaseConnection,
    pub jwk: DecodingKey,
    pub requests: Client,
}
