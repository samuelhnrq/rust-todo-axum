use crate::{clerk_user, config::LOADED_CONFIG, get_cookie_value, safe_cookie, state::HyperTarot};
use axum::{
    extract::{ConnectInfo, Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use axum_extra::extract::PrivateCookieJar;
use entity::users;
use jsonwebtoken::{
    decode,
    errors::ErrorKind,
    jwk::{Jwk, JwkSet, PublicKeyUse},
    Algorithm, DecodingKey, Validation,
};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    url::Url,
    AuthorizationCode, ClientId, ClientSecret, IssuerUrl, OAuth2TokenResponse, PkceCodeVerifier,
    RedirectUrl, RefreshToken,
};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Mutex,
};

pub const REDIRECT_PATH: &str = "/auth/redirect";

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

/// # Returns
async fn exchange_refresh_token(client: &CoreClient, refresh_token: String) -> Option<String> {
    client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(openidconnect::reqwest::async_http_client)
        .await
        .map(|z| z.access_token().secret().clone())
        .ok()
}

async fn validate_cookie(
    jar_m: &mut Mutex<PrivateCookieJar>,
    state: &HyperTarot,
) -> Option<UserData> {
    log::debug!("Getting auth cookie");
    let jwt = get_cookie_value("token", jar_m.get_mut().ok()?);
    if jwt.is_empty() {
        return None;
    }
    log::debug!("Cookie found, validating");
    match decode::<UserData>(&jwt, &state.jwk, &build_validation()) {
        Ok(session) => {
            log::debug!("Validated successfully adding extension to request");
            Some(session.claims)
        }
        Err(err) => {
            log::debug!("Token did not pass validation {:?}", err);
            if *err.kind() == ErrorKind::ExpiredSignature {
                log::debug!("JWT is expired, attempting to refresh");
                let refresh_token = get_cookie_value("refresh_token", jar_m.get_mut().ok()?);
                let refreshed = exchange_refresh_token(&state.auth_client, refresh_token).await?;
                log::debug!("Successfully refreshed, persisting the new token");
                let jar = jar_m.get_mut().ok()?;
                *jar = jar.clone().add(safe_cookie("token", refreshed));
                let decoded = decode::<UserData>(&jwt, &state.jwk, &build_validation());
                return decoded.map(|x| x.claims).ok();
            }
            None
        }
    }
}

pub async fn user_data_extension(
    jar: PrivateCookieJar,
    State(state): State<HyperTarot>,
    mut request: Request,
    next: Next,
) -> Result<(PrivateCookieJar, Response), Response> {
    let mut jar_cell = Mutex::new(jar);
    if let Some(user_data) = validate_cookie(&mut jar_cell, &state).await {
        request.extensions_mut().insert(user_data);
    }
    if let Ok(final_jar) = jar_cell.into_inner() {
        Ok((final_jar, next.run(request).await))
    } else {
        Err(next.run(request).await)
    }
}

fn build_validation() -> Validation {
    let mut val = Validation::new(Algorithm::RS256);
    val.set_audience(&["hyper-tarot"]);
    val
}

/// # Panics
/// Somethign
pub async fn core_client_factory() -> (CoreClient, CoreProviderMetadata) {
    let issuer_url = IssuerUrl::from_url(LOADED_CONFIG.oauth_autodiscover_url.clone());
    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client)
        .await
        .expect("Failed to fetch OpenID metadata");
    let mut url = Url::parse(LOADED_CONFIG.host_name.as_str()).unwrap();
    url.set_path(REDIRECT_PATH);
    let client = CoreClient::from_provider_metadata(
        provider_metadata.clone(),
        ClientId::new(LOADED_CONFIG.oauth_client_id.clone()),
        Some(ClientSecret::new(LOADED_CONFIG.oauth_client_secret.clone())),
    )
    .set_redirect_uri(RedirectUrl::from_url(url));
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
pub async fn handle_oauth_redirect(
    State(state): State<HyperTarot>,
    query: Query<AuthRedirectQuery>,
    cookies: PrivateCookieJar,
) -> impl IntoResponse {
    let crsf_token = get_cookie_value("crsf_token", &cookies);
    let pkce = get_cookie_value("pkce", &cookies);
    if query.state != crsf_token {
        log::error!(
            "CRSF attack?! state {}, stored cookie {}",
            query.state,
            crsf_token
        );
        return (cookies, Redirect::to(LOADED_CONFIG.host_name.as_str()));
    }
    let response = state
        .auth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce))
        .request_async(openidconnect::reqwest::async_http_client)
        .await;
    let session_jar = match response {
        Ok(code) => {
            let jar = cookies.add(safe_cookie("token", code.access_token().secret()));
            if let Some(refresh_token) = code.refresh_token() {
                jar.add(safe_cookie("refresh_token", refresh_token.secret()))
            } else {
                jar
            }
        }
        Err(err) => {
            log::error!("Failed to exchange token {:?}", err);
            cookies
        }
    };
    (session_jar, Redirect::to(LOADED_CONFIG.host_name.as_str()))
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
