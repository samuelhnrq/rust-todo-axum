use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use jsonwebtoken::DecodingKey;
use openidconnect::core::CoreClient;
use reqwest::Client;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct HyperTarot {
    pub connection: DatabaseConnection,
    pub jwk: DecodingKey,
    pub requests: Client,
    pub auth_client: CoreClient,
    pub key: Key,
}

// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<HyperTarot> for Key {
    fn from_ref(state: &HyperTarot) -> Self {
        state.key.clone()
    }
}
