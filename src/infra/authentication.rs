use jsonwebtoken::jwk::{Jwk, JwkSet};

pub async fn authentication_middleware() {}

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
