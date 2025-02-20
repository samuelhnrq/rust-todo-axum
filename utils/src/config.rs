use config::{builder::DefaultState, Config as Configurer, ConfigBuilder, Environment, File};
use std::{
  fs::{self, DirEntry},
  sync::LazyLock,
};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
  pub host_name: String,
  pub database_url: String,
  pub oauth_autodiscover_url: String,
  pub oauth_client_id: String,
  pub oauth_audience: String,
  pub cookie_secret: String,
  pub oauth_client_secret: String,
  #[serde(default)]
  pub www_static_files: String,
}

fn to_file_stem(entry: &DirEntry) -> Option<String> {
  let path = entry.path();
  if !path.is_file() {
    return None;
  }
  let map = path.file_stem().map(|z| z.to_string_lossy().into_owned())?;
  if map.starts_with("config") {
    Some(map)
  } else {
    None
  }
}

fn add_config_file(config_builder: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
  let file_exists: Option<String> = fs::read_dir(".")
    .map(|file_list| file_list.flatten().find_map(|entry| to_file_stem(&entry)))
    .ok()
    .unwrap_or_default();
  if let Some(file) = file_exists {
    log::info!("Config file found, '{}' loading", file);
    config_builder.add_source(File::with_name(&file))
  } else {
    log::info!("Config file not found, skipping");
    config_builder
  }
}

/// # Panics
/// If missing configuration
pub static LOADED_CONFIG: LazyLock<Config> = LazyLock::new(|| {
  let mut config_builder = Configurer::builder().add_source(Environment::with_prefix("HT"));
  config_builder = add_config_file(config_builder);
  let config = config_builder
    .build()
    .expect("Failed loading configuration sources");
  config
    .try_deserialize::<Config>()
    .expect("Failed to load config data")
});
