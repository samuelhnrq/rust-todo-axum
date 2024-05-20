use axum_extra::extract::cookie::Key;
use reqwest::Client;
use utils::authentication::{core_client_factory, fetch_remote_jwk};
use utils::config::LOADED_CONFIG;
use utils::state::HyperTarot;

use crate::adapters::database;

pub async fn create() -> HyperTarot {
    let connection = database::connect().await;
    let (auth_client, metadata) = core_client_factory().await;
    let jwk = fetch_remote_jwk(metadata.jwks_uri().to_string()).await;
    let cookie_secret = &LOADED_CONFIG.cookie_secret;
    assert!(
        !cookie_secret.is_empty(),
        "Cannot build state with missing cookie secret"
    );
    let key = Key::from(cookie_secret.as_bytes());

    HyperTarot {
        connection,
        jwk,
        auth_client,
        key,
        requests: Client::new(),
    }
}
