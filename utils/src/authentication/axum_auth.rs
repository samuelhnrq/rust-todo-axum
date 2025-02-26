use std::{
  collections::HashMap,
  net::{IpAddr, SocketAddr},
};

use axum::{
  extract::{ConnectInfo, Query, Request, State},
  http::StatusCode,
  middleware::Next,
  response::{IntoResponse, Redirect, Response},
  Json,
};
use axum_extra::extract::PrivateCookieJar;
use either::Either;
use entity::users::find_by_sub;

use crate::{
  authentication::generate_auth_url, config::LOADED_CONFIG, get_cookie_value, safe_cookie,
  state::HyperTarot,
};

use super::{
  exchange_token, from_redirect_to_token_payload,
  models::{AuthRedirectQuery, Claims},
  user_info_to_db, validate_cookie,
};

#[allow(clippy::unused_async)]
#[axum::debug_handler]
pub async fn login_handler(
  State(state): State<HyperTarot>,
  jar: PrivateCookieJar,
) -> impl IntoResponse {
  log::info!("starting login");
  let (new_jar, url) = generate_auth_url(jar, &state.oauth_config);
  log::info!("generared auth url, redirecting");
  (new_jar, Redirect::temporary(&url))
}

#[axum::debug_handler]
pub async fn logout_handler(
  State(state): State<HyperTarot>,
  jar: PrivateCookieJar,
) -> impl IntoResponse {
  log::info!("starting logout");
  let mut params = HashMap::<&str, String>::new();
  params.insert("client_id", LOADED_CONFIG.oauth_client_id.to_string());
  if let Some(refresh_token) = jar.get("refresh_token") {
    log::info!("has refresh adding to logout payload");
    params.insert("refresh_token", refresh_token.value_trimmed().to_string());
  }
  let resp = state
    .requests
    .post(state.oauth_config.end_session_endpoint)
    .basic_auth(
      &LOADED_CONFIG.oauth_client_id,
      Some(&LOADED_CONFIG.oauth_client_secret),
    )
    .form(&params)
    .send()
    .await
    .inspect_err(|err| log::error!("Failed to revoke endpoint {:?}", err))
    .ok();
  let jar = if let Some(x) = resp {
    let body = x.text().await.unwrap_or_default();
    log::info!("Got successfull answer! {body}");
    jar
      .remove(safe_cookie("token", ""))
      .remove(safe_cookie("refresh_token", ""))
  } else {
    jar
  };
  (jar, Redirect::to("/"))
}

#[axum::debug_handler]
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
      "CRSF attack?! state '{}', stored cookie '{}'",
      query.state,
      crsf_token
    );
    return (cookies, Redirect::to(&LOADED_CONFIG.host_name));
  }
  log::info!("cookies pass, converting to token exchage payload");
  let token_payload = from_redirect_to_token_payload(query, pkce);
  let response = exchange_token(&state, &Either::Left(token_payload)).await;
  let session_jar = match response {
    Ok(code) => {
      log::debug!("Exchanged token successfully, persisting token in cookies");
      let mut jar = cookies.add(safe_cookie("token", &code.access_token));
      user_info_to_db(&code.access_token, &state)
        .await
        .inspect_err(|err| log::error!("Failed to persist JWT into DB {:?}", err))
        .inspect(|user| log::info!("Successfully copied {} to database", user.oauth_sub))
        .ok();
      if let Some(refresh_token) = &code.refresh_token {
        log::debug!("Token has refresh, persisting too");
        jar = jar.add(safe_cookie("refresh_token", refresh_token));
      } else {
        log::debug!("Token has no refresh, persisting what we have");
      }
      jar
    }
    Err(err) => {
      log::error!("Failed to exchange token {:?}", err);
      cookies
    }
  };
  (session_jar, Redirect::temporary("/"))
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
    let db_conn = &state.connection;
    if let Some(user) = find_by_sub(db_conn, &user_data.sub).await {
      request.extensions_mut().insert(user);
      request.extensions_mut().insert(user_data);
      log::debug!("Inserted extensions successfully");
    } else {
      log::error!("JWT valid bug sub not found in database, not trusting cookie");
    }
  } else {
    log::debug!("Cookie did not pass validation");
  }
  (jar, next.run(request).await)
}
