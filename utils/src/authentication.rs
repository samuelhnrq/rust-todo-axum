use std::net::{IpAddr, SocketAddr};

use crate::{clerk_user, config::LOADED_CONFIG, state::HyperTarot};
use axum::{
    extract::{ConnectInfo, Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, CookieJar},
    PrivateCookieJar,
};
use entity::users;
use jsonwebtoken::{
    decode,
    jwk::{Jwk, JwkSet, PublicKeyUse},
    Algorithm, DecodingKey, Validation,
};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    AuthorizationCode, ClientId, ClientSecret, IssuerUrl, OAuth2TokenResponse,
};

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
pub struct UserData {
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
    let user_option = request.extensions().get::<UserData>();
    if is_safe_requester(addr) || user_option.is_some() {
        next.run(request).await
    } else {
        build_unauthorized_response("Unauthorized")
    }
}

pub async fn assert_in_database(state: HyperTarot, jwt: &String, user: &UserData) {
    let user_resp = clerk_user::fetch_user(&state.requests, &user.sub, jwt).await;
    if let Ok(user) = user_resp {
        users::upsert(user.into(), &state.connection)
            .await
            .err()
            .inspect(|err| log::error!("Failed to do stuff {:?}", err));
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
    let decoded = decode::<UserData>(&jwt, &state.jwk, &build_validation());
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
/// Somethign
pub async fn core_client_factory() -> (CoreClient, CoreProviderMetadata) {
    let issuer_url = IssuerUrl::from_url(LOADED_CONFIG.oauth_autodiscover_url.clone());
    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client)
        .await
        .expect("Failed to fetch OpenID metadata");
    let client = CoreClient::from_provider_metadata(
        provider_metadata.clone(),
        ClientId::new(LOADED_CONFIG.oauth_client_id.clone()),
        Some(ClientSecret::new(LOADED_CONFIG.oauth_client_secret.clone())),
    );
    (client, provider_metadata)
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AuthRedirectQuery {
    pub state: String,
    pub session_state: String,
    pub iss: String,
    pub code: String,
}

#[axum_macros::debug_handler]
pub async fn handle_redirect(
    State(state): State<HyperTarot>,
    query: Query<AuthRedirectQuery>,
    cookies: PrivateCookieJar,
) -> impl IntoResponse {
    let parts = cookies
        .get("auth_tokens")
        .map(|x| x.value().to_string())
        .unwrap_or_default();
    let tokens: Vec<&str> = parts.split('#').collect();
    if tokens.len() != 3 {
        log::info!("Failed to do stuff");
        return (cookies, Redirect::temporary("http://localhost:8080/"));
    }
    let response = state
        .auth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(openidconnect::reqwest::async_http_client)
        .await;
    let final_jar = match response {
        Ok(code) => cookies.add(Cookie::new("amazing", code.access_token().secret().clone())),
        Err(err) => {
            log::error!("Could not exchange {:?}", err);
            cookies
        }
    };
    (final_jar, Redirect::temporary("http://localhost:8080/"))
}

/// # Panics
/// if cant get the JWKS
pub async fn fetch_remote_jwk(jwks_url: String) -> DecodingKey {
    log::info!("Fetching JWKS remotely");
    let resp = reqwest::get(jwks_url)
        .await
        .expect("Failed to rearch clerk, invalid URL?")
        .json::<JwkSet>()
        .await
        .expect("Failed to deserialize JWKS response");
    log::info!("Fetched JWKS successfully");
    let jwk = resp
        .keys
        .iter()
        .find(|&x| is_sig_key(x))
        .expect("JWKS without any sig keys?!?!");
    DecodingKey::from_jwk(jwk).unwrap()
}

fn is_sig_key(key: &Jwk) -> bool {
    key.common
        .public_key_use
        .as_ref()
        .is_some_and(|k| *k == PublicKeyUse::Signature)
}
