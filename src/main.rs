use crate::config::config::ConfigLoader;
use crate::services::users_service::UserService;
use crate::app::app::run_app;

use env_logger::{Builder, Env};
use log::info;

mod config;
mod db;
mod entities;
mod services;
mod app;

#[tokio::main]
async fn main() {
    let config = config::config::Settings::load_config(Some("config.yml".to_string())).unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    info!("Starting authenticator...");
    let user_service = UserService::from_config(config.db).await;
    run_app(user_service, config.app).await;
}
