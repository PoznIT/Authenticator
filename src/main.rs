use crate::app::app::run_app;
use crate::config::config::ConfigLoader;
use crate::services::users_service::UserService;
use log::info;

mod config;
mod db;
mod entities;
mod services;
mod app;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config_path = if args.len() > 1 {
        &args[1]
    } else {
        "config-dev.yml"
    };
    let config = config::config::Settings::load_config(Some(config_path.to_string())).unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Running db migrations...");

    info!("Starting authenticator...");
    let user_service = UserService::from_config(config.db).await;
    run_app(user_service, config.app).await;
}
