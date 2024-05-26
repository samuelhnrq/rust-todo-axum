use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use jsonwebtoken::DecodingKey;
use reqwest::Client;
use sea_orm::DatabaseConnection;

use crate::authentication::models::OpenIdConfiguration;

#[derive(Clone)]
pub struct HyperTarot {
    pub connection: DatabaseConnection,
    pub oauth_config: OpenIdConfiguration,
    pub jwk: DecodingKey,
    pub requests: Client,
    pub key: Key,
}

impl FromRef<HyperTarot> for Key {
    fn from_ref(state: &HyperTarot) -> Self {
        state.key.clone()
    }
}
