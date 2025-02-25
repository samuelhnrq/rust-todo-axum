use axum_extra::extract::{
  cookie::{Cookie, SameSite},
  PrivateCookieJar,
};

pub mod authentication;
pub mod config;
pub mod state;

#[must_use]
pub(crate) fn safe_cookie<'a, K, V>(key: K, val: V) -> Cookie<'a>
where
  K: Into<String>,
  V: Into<String>,
{
  Cookie::build((key.into(), val.into()))
    .path("/")
    .http_only(true)
    .same_site(SameSite::Lax)
    .secure(true)
    .build()
}

pub(crate) fn get_cookie_value(key: &str, jar: &PrivateCookieJar) -> String {
  jar
    .get(key)
    .map(|x| x.value_trimmed().to_string())
    .unwrap_or_default()
}
