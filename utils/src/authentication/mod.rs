use std::error::Error;

use crate::{config::LOADED_CONFIG, get_cookie_value, safe_cookie, state::HyperTarot};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    PrivateCookieJar,
};
use either::{for_both, Either};
use jsonwebtoken::{
    decode,
    errors::ErrorKind,
    jwk::{Jwk, JwkSet, PublicKeyUse},
    Algorithm, DecodingKey, Validation,
};
use models::{UserData, REDIRECT_PATH};
use reqwest::{header::CONTENT_TYPE, Client, Url};

mod axum_auth;
pub mod models;
pub use axum_auth::*;

use self::models::{
    build_redirect_url, AuthRedirectQuery, AuthorizationParams, OpenIdConfiguration,
    RefreshPayload, TokenExchangePayload, TokenResponse,
};

#[must_use]
fn safe_redirect_cookie<'a, K, V>(key: K, val: V) -> Cookie<'a>
where
    K: Into<String>,
    V: Into<String>,
{
    Cookie::build((key.into(), val.into()))
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(true)
        .path(REDIRECT_PATH)
        .build()
}

async fn validate_cookie(jar: &mut PrivateCookieJar, state: &HyperTarot) -> Option<UserData> {
    let jwt = get_cookie_value("token", jar);
    if jwt.is_empty() {
        log::debug!("Missing auth cookie");
        return None;
    }
    log::debug!("Cookie found, validating");
    log::info!("jwt is {jwt}");
    match decode::<UserData>(&jwt, &state.jwk, &build_validation()) {
        Ok(session) => {
            log::debug!("Validated successfully adding extension to request");
            Some(session.claims)
        }
        Err(err) => {
            log::debug!("Token did not pass validation {:?}", err);
            if *err.kind() == ErrorKind::ExpiredSignature {
                log::debug!("JWT is expired, attempting to refresh");
                let refresh_token = get_cookie_value("refresh_token", jar);
                let payload = from_refresh_to_token_payload(refresh_token);
                let refreshed = exchange_token(state, &Either::Right(payload))
                    .await
                    .inspect_err(|err| log::error!("Failed to refresh token: '{:?}'", err))
                    .ok()?;
                log::debug!("Successfully refreshed, persisting the new token");
                *jar = jar
                    .clone()
                    .add(safe_cookie("token", &refreshed.access_token));
                let decoded = decode::<UserData>(&jwt, &state.jwk, &build_validation());
                return decoded.map(|x| x.claims).ok();
            }
            None
        }
    }
}

fn build_validation() -> Validation {
    let mut val = Validation::new(Algorithm::RS256);
    val.set_audience(&[&LOADED_CONFIG.oauth_audience]);
    val
}

/// # Panics
/// if it fails to fetch the config remotly
pub async fn load_openid_config(request: &Client) -> OpenIdConfiguration {
    let url = &LOADED_CONFIG.oauth_autodiscover_url;
    let trimmed = url.strip_suffix('/').unwrap_or(url);
    let issuer_url = Url::parse(&format!("{trimmed}/.well-known/openid-configuration"))
        .expect("Invalid oauth config URL");
    log::info!("Fetching oauth config at {}", issuer_url);
    request
        .get(issuer_url)
        .send()
        .await
        .expect("Failed to fetch oauth config URL")
        .json::<OpenIdConfiguration>()
        .await
        .expect("Failed to deserialized oauth response")
}

/// # Panics
/// if cant get the JWKS
pub async fn fetch_remote_jwk(request: &Client, config: &OpenIdConfiguration) -> DecodingKey {
    log::info!("Fetching JWKS remotely");
    let resp = request
        .get(&config.jwks_uri)
        .send()
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

fn from_redirect_to_token_payload(value: AuthRedirectQuery, pkce: String) -> TokenExchangePayload {
    TokenExchangePayload {
        code: value.code,
        client_id: LOADED_CONFIG.oauth_client_id.clone(),
        client_secret: LOADED_CONFIG.oauth_client_secret.clone(),
        code_verifier: pkce,
        grant_type: "authorization_code".to_string(),
        redirect_uri: build_redirect_url(),
    }
}

fn from_refresh_to_token_payload(token: String) -> RefreshPayload {
    RefreshPayload {
        refresh_token: token,
        client_id: LOADED_CONFIG.oauth_client_id.clone(),
        client_secret: LOADED_CONFIG.oauth_client_secret.clone(),
        grant_type: "refresh_token".to_string(),
        redirect_uri: build_redirect_url(),
    }
}

async fn exchange_token(
    state: &HyperTarot,
    payload: &Either<TokenExchangePayload, RefreshPayload>,
) -> Result<TokenResponse, Box<dyn Error>> {
    let client_id = for_both!(payload, x => &x.client_id);
    let client_secret = for_both!(payload, x => &x.client_secret);
    let body = for_both!(payload, x => serde_urlencoded::to_string(x)).map_err(Box::new)?;
    let response = state
        .requests
        .post(&state.oauth_config.token_endpoint)
        .body(body)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .basic_auth(client_id, Some(client_secret))
        .send()
        .await
        .map_err(Box::new)?;
    let body = response.text().await.map_err(Box::new)?;
    serde_json::from_str(&body)
        .inspect_err(|err| log::error!("failed to deserialize '{}', error: {:?}", body, err))
        .map_err(|err| -> Box<dyn Error> { Box::new(err) })
}

pub fn generate_auth_url(jar: &mut PrivateCookieJar, config: &OpenIdConfiguration) -> String {
    let params = AuthorizationParams::new();
    *jar = jar
        .clone()
        .add(safe_redirect_cookie("pkce", &params.code_verifier))
        .add(safe_redirect_cookie("crsf", &params.state));
    Url::parse(&config.authorization_endpoint)
        .map(|mut x| {
            x.set_query(
                serde_urlencoded::to_string(params)
                    .inspect_err(|err| log::error!("Failed to url-encode auth params {}", err))
                    .ok()
                    .as_deref(),
            );
            x.to_string()
        })
        .unwrap_or("Failed to generate URL".to_string())
}

fn is_sig_key(key: &Jwk) -> bool {
    key.common
        .public_key_use
        .as_ref()
        .is_some_and(|k| *k == PublicKeyUse::Signature)
}
