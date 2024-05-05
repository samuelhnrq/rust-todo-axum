use tower_http::services::ServeDir;

pub fn build_service() -> ServeDir {
    let dest = std::env::var("WWW_STATIC_FILES").unwrap_or("variable not set".into());
    let resolved = std::fs::canonicalize(&dest).unwrap_or_default();
    let dest_exists = std::fs::metadata(&resolved).is_ok();
    if !dest_exists {
        log::warn!("$WWW_STATIC_FILES '{}' folder not found", dest);
    }
    ServeDir::new(resolved)
}
