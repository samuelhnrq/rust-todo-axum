use base64ct::{Base64Url, Encoding};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::config::LOADED_CONFIG;

use super::REDIRECT_PATH;

#[derive(Debug, Clone, Deserialize)]
pub struct OpenIdConfiguration {
    pub issuer: String,
    pub jwks_uri: String,
    pub authorization_endpoint: String,
    pub backchannel_logout_supported: bool,
    pub frontchannel_logout_supported: bool,
    pub grant_types_supported: Vec<String>,
    pub response_modes_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub token_endpoint: String,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub userinfo_endpoint: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthRedirectQuery {
    pub state: String,
    pub session_state: String,
    pub iss: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenExchangePayload {
    pub code: String,
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub code_verifier: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RefreshPayload {
    pub grant_type: String,
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: Option<i64>,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub id_token: String,
    pub session_state: String,
    pub scope: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct AuthorizationParams {
    pub client_id: String,
    pub redirect_uri: String,
    pub state: String,
    pub response_mode: String,
    pub response_type: String,
    pub scope: String,
    pub nonce: String,
    #[serde(skip)]
    pub code_verifier: String,
    pub code_challenge: String,
    pub code_challenge_method: String,
}

#[must_use]
pub fn build_redirect_url() -> String {
    Url::parse(&LOADED_CONFIG.host_name)
        .map(|mut x| {
            x.set_path(REDIRECT_PATH);
            x.to_string()
        })
        .unwrap_or_default()
}

fn generate_pkce() -> String {
    let pkce_verifier = Uuid::new_v4();
    format!("{pkce_verifier}-{pkce_verifier}")
}

impl AuthorizationParams {
    #[must_use]
    pub fn new() -> Self {
        let pkce_verifier = generate_pkce();
        let mut hasher = Sha256::new();
        hasher.update(&pkce_verifier);
        let padded_pkce_challenge = Base64Url::encode_string(&hasher.finalize());
        let pkce_challenge = padded_pkce_challenge
            .strip_suffix('=')
            .unwrap_or(&padded_pkce_challenge);
        log::debug!("PKCE is {} challenge is {}", pkce_verifier, pkce_challenge);
        let crsf = Uuid::new_v4();
        let nonce = Uuid::new_v4();
        AuthorizationParams {
            client_id: LOADED_CONFIG.oauth_client_id.clone(),
            redirect_uri: build_redirect_url(),
            state: crsf.to_string(),
            response_mode: "query".to_string(),
            response_type: "code".to_string(),
            scope: "openid".to_string(),
            nonce: nonce.to_string(),
            code_verifier: pkce_verifier.to_string(),
            code_challenge: pkce_challenge.to_string(),
            code_challenge_method: "S256".to_string(),
        }
    }
}
