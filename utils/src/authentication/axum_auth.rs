use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{ConnectInfo, Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use axum_extra::extract::PrivateCookieJar;
use either::Either;

use crate::{config::LOADED_CONFIG, get_cookie_value, safe_cookie, state::HyperTarot};

use super::{
    copy_to_db, exchange_token, from_redirect_to_token_payload,
    models::{AuthRedirectQuery, Claims},
    validate_cookie,
};

#[axum_macros::debug_handler]
pub async fn handle_oauth_redirect(
    State(state): State<HyperTarot>,
    Query(query): Query<AuthRedirectQuery>,
    cookies: PrivateCookieJar,
) -> impl IntoResponse {
    log::info!("Got oauth2 redirect, reading cookies");
    let crsf_token = get_cookie_value("crsf", &cookies);
    let pkce = get_cookie_value("pkce", &cookies);
    if query.state != crsf_token {
        log::error!(
            "CRSF attack?! state {}, stored cookie {}",
            query.state,
            crsf_token
        );
        return (cookies, Redirect::to(LOADED_CONFIG.host_name.as_str()));
    }
    log::info!("cookies pass, converting to token exchage payload");
    let token_payload = from_redirect_to_token_payload(query, pkce);
    let response = exchange_token(&state, &Either::Left(token_payload)).await;
    let session_jar = match response {
        Ok(code) => {
            log::debug!("Exchanged token successfully, persisting token in cookies");
            let jar = cookies.add(safe_cookie("token", &code.access_token));
            copy_to_db(code.access_token.clone(), &state)
                .await
                .inspect_err(|err| log::error!("Failed to persist JWT into DB {:?}", err))
                .inspect(|user| log::info!("Successfully copied {} to database", user.name))
                .ok();
            if let Some(refresh_token) = &code.refresh_token {
                log::debug!("Token has refresh, persisting too");
                jar.add(safe_cookie("refresh_token", refresh_token))
            } else {
                jar
            }
        }
        Err(err) => {
            log::error!("Failed to exchange token {:?}", err);
            cookies
        }
    };
    (session_jar, Redirect::to(LOADED_CONFIG.host_name.as_str()))
}

#[derive(serde::Serialize)]
struct UnauthorizedError {
    code: &'static str,
    message: &'static str,
}

impl Default for UnauthorizedError {
    fn default() -> Self {
        UnauthorizedError::new("Unauthorized")
    }
}

impl UnauthorizedError {
    pub(crate) fn new(message: &'static str) -> Self {
        UnauthorizedError {
            code: "UNAUTHORIZED",
            message,
        }
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
    let user_option = request.extensions().get::<Claims>();
    if is_safe_requester(addr) || user_option.is_some() {
        next.run(request).await
    } else {
        build_unauthorized_response("Unauthorized")
    }
}

pub async fn user_data_extension(
    mut jar: PrivateCookieJar,
    State(state): State<HyperTarot>,
    mut request: Request,
    next: Next,
) -> (PrivateCookieJar, Response) {
    if let Some(user_data) = validate_cookie(&mut jar, &state).await {
        request.extensions_mut().insert(user_data);
    }
    (jar, next.run(request).await)
}
