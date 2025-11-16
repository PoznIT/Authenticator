use crate::config::db_config::DbConfig;
use config::{Config, File, FileFormat};
use serde::Deserialize;
use std::env;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub db_config: DbConfig,
}

pub trait ConfigLoader {
    fn load_config(config_path: Option<String>) -> Result<Settings, config::ConfigError>;
    fn get_config_path(config_path: Option<String>) -> String {
        let unwrapped_config_path = config_path
            .unwrap_or_else(|| env::var("CONFIG_PATH").unwrap_or_else(|_| "config".to_string()));
        if !Path::new(&unwrapped_config_path).exists() {
            panic!("Config path does not exist: {}", unwrapped_config_path);
        }
        unwrapped_config_path
    }
}

impl ConfigLoader for Settings {
    fn load_config(config_path: Option<String>) -> Result<Settings, config::ConfigError> {
        Config::builder()
            .add_source(File::new(
                &*Self::get_config_path(config_path),
                FileFormat::Yaml,
            ))
            .build()?
            .try_deserialize()
    }
}
