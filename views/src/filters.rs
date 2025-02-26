use std::fmt::Display;

type RResult<T> = rinja::Result<T>;

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn ok_or_default<T: Display + Default>(s: Option<&T>) -> RResult<String> {
  if let Some(s) = s {
    return RResult::Ok(s.to_string());
  }
  RResult::Ok(T::default().to_string())
}
