use rinja::Template;

#[derive(Template)]
#[template(path = "error.jinja.html")]
pub(crate) struct ErrorTemplate {
  error: String,
}

impl ErrorTemplate {
  pub(crate) fn new(error: String) -> Self {
    Self { error }
  }
}
