use axum::{extract::State, http::HeaderMap, Extension};
use axum_extra::extract::PrivateCookieJar;
use entity::generated::users;
use maud::{html, Markup};
use utils::{
    authentication::{
        generate_auth_url,
        models::{OpenIdConfiguration, LOGOUT_PATH},
    },
    state::HyperTarot,
};

#[axum_macros::debug_handler]
pub async fn fragment_controller(
    State(state): State<HyperTarot>,
    maybe_user: Option<Extension<users::Model>>,
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
        "max-age=1,must-revalidate,private".parse().unwrap(),
    );
    (jar, headers, html_result)
}

struct AuthParams<'a> {
    jar: &'a mut PrivateCookieJar,
    auth_config: &'a OpenIdConfiguration,
    user: Option<users::Model>,
}

fn login_button(params: &mut AuthParams) -> Markup {
    let url = generate_auth_url(params.jar, params.auth_config);
    html! {
        a href=(url) hx-boost="false" id="login-anchor" { "Do login" }
    }
}

fn user_avatar(user: &users::Model) -> Markup {
    // let url = generate_logout_url(params.auth_config);
    html! {
        #user-avatar {
            span { "Welcome " (user.name) "! " }
            a href=(LOGOUT_PATH) hx-boost="false" { "Logout" }
        }
    }
}

fn user_auth(params: &mut AuthParams) -> Markup {
    html! {
        @match params.user.clone() {
            Some(user) => (user_avatar(&user)) ,
            None => (login_button(params)),
        }
    }
}
