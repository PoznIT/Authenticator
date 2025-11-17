use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::{serve, Router};
use log::info;
use crate::app::routes::users_routes::init_routes;
use crate::config::app_config::AppConfig;
use crate::services::users_service::UserService;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
}

pub async fn run_app(user_service: UserService, app_config: AppConfig) {
    let app_state = AppState { user_service };
    let router = Router::new()
        .nest("/users", init_routes())
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", app_config.host, app_config.port)
        .parse()
        .unwrap();

    info!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, router).await.unwrap();
}
