use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub port: String,
    pub host: String,
}