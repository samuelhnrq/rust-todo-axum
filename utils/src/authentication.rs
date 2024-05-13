use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use crate::clerk_user;
use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use entity::{users, HyperTarot};
use jsonwebtoken::{
    decode,
    jwk::{Jwk, JwkSet, PublicKeyUse},
    Algorithm, DecodingKey, Validation,
};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    AccessToken, ClientId, ClientSecret, EmptyAdditionalClaims, GenderClaim, IdToken, IssuerUrl,
    StandardClaims, SubjectIdentifier, UserInfoClaims,
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

pub async fn assert_in_database(state: HyperTarot, jwt: &String, user: &UserClaims) {
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

pub async fn core_client_factory() -> CoreClient {
    // http://localhost:8888/realms/master/.well-known/openid-configuration
    let issuers = env::var("OPENID_AUTODISCOVER").expect("Missing $OPENID_AUTODISCOVER");
    let issuer_url = IssuerUrl::new(issuers).expect("Inavalid $OPENID_AUTODISCOVER url");
    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client)
        .await
        .expect("Failed to fetch OpenID metadata");
    let client_id = env::var("OAUTH_CLIENT_ID").expect("Missing $OAUTH_CLIENT_ID");
    let client_secret = env::var("OAUTH_CLIENT_SECRET").expect("Missing $OAUTH_CLIENT_SECRET");
    // Create an OpenID Connect client by specifying the client ID, client secret, authorization URL
    // and token URL.
    CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    )
}

/// # Panics
/// if cant get the JWKS
pub async fn fetch_remote_jwk() -> DecodingKey {
    let jwks_url = std::env::var("JWKS_URL").expect("Missing $JWKS_URI");
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
        .filter(is_sig_key)
        .next()
        .expect("JWKS without any sig keys?!?!");
    DecodingKey::from_jwk(jwk).unwrap()
}

fn is_sig_key(key: &&Jwk) -> bool {
    key.common
        .public_key_use
        .as_ref()
        .is_some_and(|k| *k == PublicKeyUse::Signature)
}
