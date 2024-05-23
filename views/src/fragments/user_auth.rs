use std::cell::RefCell;

use axum::{extract::State, http::HeaderMap, Extension};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    PrivateCookieJar,
};
use maud::{html, Markup};
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient},
    CsrfToken, Nonce, PkceCodeChallenge, Scope,
};
use utils::{
    authentication::{UserData, REDIRECT_PATH},
    state::HyperTarot,
};

#[axum_macros::debug_handler]
pub async fn fragment_controller(
    State(state): State<HyperTarot>,
    maybe_user: Option<Extension<UserData>>,
    jar: PrivateCookieJar,
) -> (PrivateCookieJar, HeaderMap, Markup) {
    let boxed_jar = RefCell::new(jar);
    let params = AuthParams {
        jar: &boxed_jar,
        oauth_client: &state.auth_client,
        user: maybe_user.map(|Extension(user)| user),
    };
    let html_result = user_auth(&params);
    let mut headers = HeaderMap::new();
    headers.insert(
        "Cache-Control",
        "max-age=5,must-revalidate,private".parse().unwrap(),
    );
    (boxed_jar.into_inner(), headers, html_result)
}

struct AuthParams<'a> {
    jar: &'a RefCell<PrivateCookieJar>,
    user: Option<UserData>,
    oauth_client: &'a CoreClient,
}

#[must_use]
pub fn safe_redirect_cookie<'a, K, V>(key: K, val: V) -> Cookie<'a>
where
    K: Into<String>,
    V: Into<String>,
{
    Cookie::build((key.into(), val.into()))
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(true)
        .path(REDIRECT_PATH)
        .build()
}

fn login_button(params: &AuthParams) -> Markup {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (url, crsf_token, nonce) = params
        .oauth_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".into()))
        .add_scope(Scope::new("offline_access".into()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    params.jar.replace_with(|old_jar| {
        old_jar
            .clone()
            .add(safe_redirect_cookie("crsf_token", crsf_token.secret()))
            .add(safe_redirect_cookie("pkce", pkce_verifier.secret()))
            .add(safe_redirect_cookie("nonce", nonce.secret()))
    });
    html! {
        a href=(url) { "Do login" }
    }
}

fn user_auth(params: &AuthParams) -> Markup {
    html! {
        @match params.user.as_ref() {
            Some(user) => div { "got user " (user.sub) } ,
            None => (login_button(params)),
        }
    }
}
