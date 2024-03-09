use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{
    decode,
    jwk::{Jwk, JwkSet},
    Algorithm, DecodingKey, Validation,
};

use crate::state::AppState;

#[derive(serde::Serialize)]
struct UnauthorizedError {
    code: &'static str,
    message: &'static str,
}

impl UnauthorizedError {
    pub(crate) fn new(message: Option<&'static str>) -> Self {
        UnauthorizedError {
            code: "UNAUTHORIZED",
            message: message.unwrap_or(""),
        }
    }
}

fn is_safe_requester(addr: SocketAddr) -> bool {
    match addr.ip() {
        IpAddr::V4(ipv4) => ipv4.is_private() || ipv4.is_loopback(),
        IpAddr::V6(ipv6) => ipv6.is_loopback(),
    }
}

fn build_unauthorized_response(cause: &'static str) -> Response {
    let error = UnauthorizedError::new(Some(cause));
    let mut unauthorized = Json(error).into_response();
    *unauthorized.status_mut() = StatusCode::FORBIDDEN;
    unauthorized
}

pub async fn authentication_middleware(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    log::debug!("Authenticating {}", addr);
    if is_safe_requester(addr) {
        log::debug!("Skipping authentication, local");
        return next.run(request).await;
    }
    log::debug!("Getting auth header");
    let auth_header = match request.headers().get("Authorization") {
        Some(x) => x.to_str().unwrap_or(""),
        None => {
            return build_unauthorized_response("Missing Authorization header");
        }
    };
    log::debug!("Splitting auth header");
    let (bearer, jwt) = match auth_header.split_once(' ') {
        Some(x) => x,
        None => {
            log::debug!("Malformed auth token");
            return build_unauthorized_response("Malformed authorization header");
        }
    };
    if bearer.to_lowercase() != "bearer" {
        log::debug!("invalid token type {}", bearer);
        return build_unauthorized_response("Bad token type");
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
            build_unauthorized_response("JWT token failed validation")
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
