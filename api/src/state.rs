use axum_extra::extract::cookie::Key;
use reqwest::Client;
use utils::authentication::{fetch_remote_jwk, load_openid_config};
use utils::config::LOADED_CONFIG;
use utils::state::HyperTarot;

use crate::adapters::database;

pub async fn create() -> HyperTarot {
    let requests = Client::builder()
        .connection_verbose(true)
        .build()
        .expect("Failed to build reqwest client");
    let connection = database::connect().await;
    let oauth_config = load_openid_config(&requests).await;
    let jwk = fetch_remote_jwk(&requests, &oauth_config).await;
    let cookie_secret = &LOADED_CONFIG.cookie_secret;
    assert!(
        !cookie_secret.is_empty(),
        "Cannot build state with missing cookie secret"
    );
    let key = Key::from(cookie_secret.as_bytes());

    HyperTarot {
        connection,
        oauth_config,
        jwk,
        requests,
        key,
    }
}
