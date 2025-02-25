use axum::{extract::rejection::ExtensionRejection, http::HeaderMap, Extension};
use entity::generated::users;
use rinja::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "auth.jinja.html")] // using the template in this path, relative
pub(crate) struct AuthTemplate {
  user: Option<users::Model>,
}

#[axum::debug_handler]
pub async fn fragment_controller(
  maybe_user: Result<Extension<users::Model>, ExtensionRejection>,
) -> (HeaderMap, AuthTemplate) {
  let html_result = AuthTemplate {
    user: maybe_user.map(|Extension(user)| user).ok(),
  };
  let mut headers = HeaderMap::new();
  headers.insert(
    "Cache-Control",
    "max-age=1,must-revalidate,private".parse().unwrap(),
  );
  (headers, html_result)
}
