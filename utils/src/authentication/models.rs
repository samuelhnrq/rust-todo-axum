use base64ct::{Base64Url, Encoding};
use entity::users::NewUser;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::config::LOADED_CONFIG;

pub const REDIRECT_PATH: &str = "/auth/redirect";

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Claims {
    pub sub: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub preferred_username: String,
    pub picture: Option<String>,
    pub email: String,
    pub birthdate: Option<String>,
    pub locale: Option<String>,
}

impl From<UserInfo> for NewUser {
    fn from(value: UserInfo) -> Self {
        NewUser {
            email: value.email,
            name: value.name,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenIdConfiguration {
    // pub(crate) issuer: String,
    pub(crate) jwks_uri: String,
    pub(crate) authorization_endpoint: String,
    // pub(crate) backchannel_logout_supported: bool,
    // pub(crate) frontchannel_logout_supported: bool,
    // pub(crate) grant_types_supported: Vec<String>,
    // pub(crate) response_modes_supported: Vec<String>,
    // pub(crate) response_types_supported: Vec<String>,
    pub(crate) token_endpoint: String,
    // pub(crate) token_endpoint_auth_methods_supported: Vec<String>,
    pub(crate) userinfo_endpoint: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthRedirectQuery {
    pub state: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct TokenExchangePayload {
    pub(crate) code: String,
    pub(crate) grant_type: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) redirect_uri: String,
    pub(crate) code_verifier: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RefreshPayload {
    pub(crate) grant_type: String,
    pub(crate) refresh_token: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) redirect_uri: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub(crate) struct TokenResponse {
    pub(crate) access_token: String,
    // pub(crate) expires_in: i64,
    // pub(crate) refresh_expires_in: Option<i64>,
    pub(crate) refresh_token: Option<String>,
    // pub(crate) token_type: String,
    // pub(crate) id_token: Option<String>,
    // pub(crate) session_state: String,
    // pub(crate) scope: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub(crate) struct AuthorizationParams {
    pub(crate) client_id: String,
    pub(crate) redirect_uri: String,
    pub(crate) state: String,
    pub(crate) audience: String,
    pub(crate) response_mode: String,
    pub(crate) response_type: String,
    pub(crate) scope: String,
    #[serde(skip)]
    pub(crate) code_verifier: String,
    pub(crate) code_challenge: String,
    pub(crate) code_challenge_method: String,
}

#[must_use]
pub(crate) fn build_redirect_url() -> String {
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
    pub(crate) fn new() -> Self {
        let pkce_verifier = generate_pkce();
        let mut hasher = Sha256::new();
        hasher.update(&pkce_verifier);
        let padded_pkce_challenge = Base64Url::encode_string(&hasher.finalize());
        let pkce_challenge = padded_pkce_challenge
            .strip_suffix('=')
            .unwrap_or(&padded_pkce_challenge);
        let crsf = Uuid::new_v4();
        AuthorizationParams {
            client_id: LOADED_CONFIG.oauth_client_id.clone(),
            redirect_uri: build_redirect_url(),
            state: crsf.to_string(),
            response_mode: "query".to_string(),
            audience: "hyper-tarot".to_string(),
            response_type: "code".to_string(),
            scope: "offline_access openid email profile".to_string(),
            code_verifier: pkce_verifier.to_string(),
            code_challenge: pkce_challenge.to_string(),
            code_challenge_method: "S256".to_string(),
        }
    }
}
