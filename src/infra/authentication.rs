use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{
    decode,
    jwk::{Jwk, JwkSet},
    Algorithm, DecodingKey, Validation,
};

use crate::state::AppState;

pub async fn authentication_middleware(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    let unauthorized = Response::builder()
        .status(200)
        .body("UNAUTHORIZED".into())
        .unwrap();
    log::debug!("Authenticating {}", addr);
    if addr.ip().is_loopback() {
        log::debug!("Skipping authentication, local");
        return next.run(request).await;
    }
    log::debug!("Getting auth header");
    let auth_header = match request.headers().get("Authorization") {
        Some(x) => x.to_str().unwrap_or(""),
        None => {
            return unauthorized;
        }
    };
    log::debug!("Splitting auth header");
    let (bearer, jwt) = match auth_header.split_once(' ') {
        Some(x) => x,
        None => {
            log::debug!("Malformed auth token");
            return unauthorized;
        }
    };
    if bearer.to_lowercase() != "bearer" {
        log::debug!("invalid token type {}", bearer);
        return unauthorized;
    }
    log::debug!("split successfully, validating");
    let decoded = decode::<serde_json::Value>(
        jwt,
        &DecodingKey::from_jwk(&state.jwk).unwrap(),
        &build_validation(),
    );
    match decoded {
        Ok(_) => next.run(request).await,
        Err(err) => {
            log::debug!("validation failed {}", err);
            unauthorized
        }
    }
}

fn build_validation() -> Validation {
    let mut val = Validation::default();
    val.set_audience(&["audieence"]);
    val.algorithms = vec![Algorithm::RS256];
    val
}

pub async fn fetch_remote_jwk() -> Jwk {
    let base_url = std::env::var("OAUTH_ISSUER").expect("Missing clerk URL");
    let jwks_url = format!("{}/.well-known/jwks.json", base_url);
    log::info!("Fetching JWKS remotely");
    let resp = reqwest::get(jwks_url)
        .await
        .expect("Failed to rearch clerk, invalid URL?")
        .json::<JwkSet>()
        .await
        .expect("Failed to deserialize clerk response");
    log::info!("Fetched JWKS successfully");
    resp.keys
        .get(0)
        .expect("JWKS without any keys?!?!")
        .to_owned()
}
