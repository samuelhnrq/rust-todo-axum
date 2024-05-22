use axum_extra::extract::cookie::{Cookie, SameSite};

pub mod authentication;
mod clerk_user;
pub mod config;
pub mod state;

#[must_use]
pub fn safe_cookie<'a, K, V>(key: K, val: V) -> Cookie<'a>
where
    K: Into<String>,
    V: Into<String>,
{
    Cookie::build((key.into(), val.into()))
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(true)
        .path("/")
        .build()
}
