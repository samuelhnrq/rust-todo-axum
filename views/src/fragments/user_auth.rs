use std::{borrow::Cow, cell::RefCell};

use axum::{
    extract::{Host, State},
    Extension,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite::Strict},
    CookieJar,
};
use entity::HyperTarot;
use maud::{html, Markup};
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient},
    url::Url,
    CsrfToken, Nonce, PkceCodeChallenge, RedirectUrl, Scope,
};
use utils::authentication::UserData;

#[axum_macros::debug_handler]
pub async fn fragment_controller(
    State(state): State<HyperTarot>,
    maybe_user: Option<Extension<UserData>>,
    jar: CookieJar,
    Host(host): Host,
) -> (CookieJar, Markup) {
    let boxed_jar = RefCell::new(jar);
    let params = AuthParams {
        jar: &boxed_jar,
        host: &Url::parse(host.as_str()).unwrap(),
        oauth_client: &state.auth_client,
        user: &maybe_user.map(|Extension(user)| user),
    };
    let html_result = user_auth(&params);
    (boxed_jar.into_inner(), html_result)
}

struct AuthParams<'a> {
    jar: &'a RefCell<CookieJar>,
    user: &'a Option<UserData>,
    host: &'a Url,
    oauth_client: &'a CoreClient,
}

fn no_user(params: &AuthParams) -> Markup {
    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    log::info!("got my rwlock ref will try to read it");
    let mut new_jar = params.jar.borrow_mut();
    log::info!("Im still alive will try to mutate its ref now");
    *new_jar = new_jar.to_owned().add(
        Cookie::build(("foo", "bar"))
            .http_only(true)
            .secure(true)
            .same_site(Strict),
    );
    log::info!("cool generating URL");
    let mut url = params.host.to_owned();
    url.set_path("/auth/redirect");
    let (url, _a, _b) = params
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
    html! {
        a href=(url) { "Do login" }
    }
}

fn user_auth(params: &AuthParams) -> Markup {
    html! {
        @match params.user {
            Some(user) => div { "got user " (user.sub) } ,
            None => (no_user(params)),
        }
    }
}
