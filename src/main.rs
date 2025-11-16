use crate::config::config::ConfigLoader;
use crate::services::users_service::UserService;

mod config;
mod db;
mod entities;
mod services;

#[tokio::main]
async fn main() {
    let config = config::config::Settings::load_config(Some("config.yml".to_string())).unwrap();
    let user_service = UserService::from_config(config.db_config).await;
    // new_user("a@a.com", "blabla", "12345678aA!", db_pool).await.unwrap();
    let auth_res = user_service
        .authenticate_user("b@a.com", "blabla", "02345678aA!")
        .await
        .unwrap();
    println!("Auth result: {:?}", auth_res);
}
