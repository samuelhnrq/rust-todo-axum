use tower_http::services::ServeDir;

pub fn static_files_service() -> ServeDir {
    let dest = std::env::var("STATIC_FILES").unwrap_or_default();
    let dest_exists = std::fs::metadata(&dest).is_ok();
    if !dest_exists {
        return ServeDir::new("fixme");
    }
    ServeDir::new(dest)
}
