use crate::config::config::ConfigLoader;
use crate::services::users_service::UserService;

mod config;
mod db;
mod entities;
mod services;
mod app;

#[tokio::main]
async fn main() {
    let config = config::config::Settings::load_config(Some("config.yml".to_string())).unwrap();
    let user_service = UserService::from_config(config.db_config).await;
   
}
