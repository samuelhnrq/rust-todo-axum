use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use entity::HyperTarot;
use jsonwebtoken::{decode, jwk::JwkSet, Algorithm, DecodingKey, Validation};

#[derive(serde::Serialize)]
struct UnauthorizedError {
    code: &'static str,
    message: &'static str,
}

impl UnauthorizedError {
    pub(crate) fn new(message: &'static str) -> Self {
        UnauthorizedError {
            code: "UNAUTHORIZED",
            message,
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct UserClaims {
    pub sub: String,
}

impl Default for UnauthorizedError {
    fn default() -> Self {
        UnauthorizedError::new("Unauthorized")
    }
}

fn is_safe_requester(addr: SocketAddr) -> bool {
    match addr.ip() {
        IpAddr::V4(ipv4) => ipv4.is_private() || ipv4.is_loopback(),
        IpAddr::V6(ipv6) => ipv6.is_loopback(),
    }
}

fn build_unauthorized_response(cause: &'static str) -> Response {
    let error = UnauthorizedError::new(cause);
    let mut unauthorized = Json(error).into_response();
    *unauthorized.status_mut() = StatusCode::FORBIDDEN;
    unauthorized
}

pub async fn required_login_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    let user_option = request.extensions().get::<UserClaims>();
    if is_safe_requester(addr) || user_option.is_some() {
        next.run(request).await
    } else {
        build_unauthorized_response("Unauthorized")
    }
}

pub async fn user_data_extension(
    State(state): State<HyperTarot>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    log::debug!("Authenticating {}", addr);
    log::debug!("Getting auth __sesion cookie");
    let jwt: String = if let Some(x) = jar.get("__session") {
        x.value_trimmed().into()
    } else {
        log::debug!("Missing __session cookie");
        return next.run(request).await;
    };
    log::debug!("Cookie found, validating");
    let decoded = decode::<UserClaims>(&jwt, &state.jwk, &build_validation());
    match decoded {
        Ok(session) => {
            log::debug!("Validated successfully adding extension to request");
            log::debug!("Data is {:?}", session.claims);
            request.extensions_mut().insert(session.claims);
        }
        Err(err) => log::debug!("Token did not pass validation {:?}", err),
    }
    next.run(request).await
}

fn build_validation() -> Validation {
    let mut val = Validation::new(Algorithm::RS256);
    val.set_audience(&["hyper-tarot-rs"]);
    val
}

/// # Panics
/// if cant get the JWKS
pub async fn fetch_remote_jwk() -> DecodingKey {
    let base_url = std::env::var("OAUTH_ISSUER").expect("Missing clerk URL");
    let jwks_url = format!("{base_url}/.well-known/jwks.json");
    log::info!("Fetching JWKS remotely");
    let resp = reqwest::get(jwks_url)
        .await
        .expect("Failed to rearch clerk, invalid URL?")
        .json::<JwkSet>()
        .await
        .expect("Failed to deserialize clerk response");
    log::info!("Fetched JWKS successfully");
    let jwk = resp.keys.first().expect("JWKS without any keys?!?!");
    DecodingKey::from_jwk(jwk).unwrap()
}
