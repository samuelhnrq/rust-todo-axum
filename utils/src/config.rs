use config::{Config as Configurer, Environment, File};
use once_cell::sync::Lazy;
use openidconnect::url::Url;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub host_name: Url,
    pub database_url: String,
    pub oauth_autodiscover_url: Url,
    pub oauth_client_id: String,
    pub cookie_secret: String,
    pub oauth_client_secret: String,
    #[serde(default)]
    pub www_static_files: String,
}

/// # Panics
/// If missing configuration
pub static LOADED_CONFIG: Lazy<Config> = Lazy::new(|| {
    let config = Configurer::builder()
        .add_source(Environment::with_prefix("CONFIG"))
        .add_source(File::with_name("config"))
        .build()
        .expect("Failed loading configuration sources");
    config
        .try_deserialize::<Config>()
        .expect("Failed to load config data")
});
