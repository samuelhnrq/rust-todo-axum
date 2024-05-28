use axum::{extract::State, http::HeaderMap, Extension};
use axum_extra::extract::PrivateCookieJar;
use maud::{html, Markup};
use utils::{
    authentication::{
        generate_auth_url,
        models::{OpenIdConfiguration, UserData},
    },
    state::HyperTarot,
};

#[axum_macros::debug_handler]
pub async fn fragment_controller(
    State(state): State<HyperTarot>,
    maybe_user: Option<Extension<UserData>>,
    mut jar: PrivateCookieJar,
) -> (PrivateCookieJar, HeaderMap, Markup) {
    let mut params = AuthParams {
        jar: &mut jar,
        auth_config: &state.oauth_config,
        user: maybe_user.map(|Extension(user)| user),
    };
    let html_result = user_auth(&mut params);
    let mut headers = HeaderMap::new();
    headers.insert(
        "Cache-Control",
        "max-age=5,must-revalidate,private".parse().unwrap(),
    );
    (jar, headers, html_result)
}

struct AuthParams<'a> {
    jar: &'a mut PrivateCookieJar,
    auth_config: &'a OpenIdConfiguration,
    user: Option<UserData>,
}

fn login_button(params: &mut AuthParams) -> Markup {
    let url = generate_auth_url(params.jar, params.auth_config);
    html! {
        a href=(url) { "Do login" }
    }
}

fn user_auth(params: &mut AuthParams) -> Markup {
    html! {
        @match params.user.as_ref() {
            Some(user) => div { "got user " (user.sub) } ,
            None => (login_button(params)),
        }
    }
}
