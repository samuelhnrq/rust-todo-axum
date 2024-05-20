use std::{borrow::Cow, cell::RefCell};

use axum::{extract::State, Extension};
use axum_extra::extract::PrivateCookieJar;
use maud::{html, Markup};
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient},
    url::Url,
    CsrfToken, Nonce, PkceCodeChallenge, RedirectUrl, Scope,
};
use utils::{authentication::UserData, config::LOADED_CONFIG, safe_cookie, state::HyperTarot};

#[axum_macros::debug_handler]
pub async fn fragment_controller(
    State(state): State<HyperTarot>,
    maybe_user: Option<Extension<UserData>>,
    jar: PrivateCookieJar,
) -> (PrivateCookieJar, Markup) {
    let boxed_jar = RefCell::new(jar);
    let params = AuthParams {
        jar: &boxed_jar,
        oauth_client: &state.auth_client,
        user: maybe_user.map(|Extension(user)| user),
    };
    let html_result = user_auth(&params);
    (boxed_jar.into_inner(), html_result)
}

struct AuthParams<'a> {
    jar: &'a RefCell<PrivateCookieJar>,
    user: Option<UserData>,
    oauth_client: &'a CoreClient,
}

fn no_user(params: &AuthParams) -> Markup {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let mut url = Url::parse(LOADED_CONFIG.host_name.as_str()).unwrap();
    url.set_path("/auth/redirect");
    let (url, crsf_token, nonce) = params
        .oauth_client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .set_redirect_uri(Cow::Owned(RedirectUrl::from_url(url)))
        .add_scope(Scope::new("openid".into()))
        .set_pkce_challenge(pkce_challenge)
        .url();
    params.jar.replace_with(|old_jar| {
        old_jar.to_owned().add(safe_cookie(
            "auth_tokens".to_string(),
            format!(
                "{}#{}#{}",
                crsf_token.secret(),
                pkce_verifier.secret(),
                nonce.secret()
            ),
        ))
    });
    html! {
        a href=(url) { "Do login" }
    }
}

fn user_auth(params: &AuthParams) -> Markup {
    html! {
        @match params.user.as_ref() {
            Some(user) => div { "got user " (user.sub) } ,
            None => (no_user(params)),
        }
    }
}
