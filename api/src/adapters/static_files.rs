use tower_http::services::ServeDir;
use utils::config::LOADED_CONFIG;

pub fn build_service() -> ServeDir {
  let dest = LOADED_CONFIG.www_static_files.clone();
  let resolved = std::fs::canonicalize(&dest).unwrap_or_default();
  let dest_exists = std::fs::metadata(&resolved).is_ok();
  if !dest_exists {
    log::warn!("$WWW_STATIC_FILES '{}' folder not found", dest);
  }
  ServeDir::new(resolved).precompressed_br()
}
