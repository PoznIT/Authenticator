use axum::extract::State;
use axum::Router;
use axum::routing::{get, post};
use crate::app::app::AppState;


async fn register_user(State(state): State<AppState>) {
    todo!()
}

async fn authenticate_user(State(state): State<AppState>) {
    todo!()
}

async fn change_user_password(State(state): State<AppState>) {
    todo!()
}

async fn delete_user(State(state): State<AppState>) {
    todo!()
}

pub fn init_user_routes(app_state: AppState) -> Router<AppState> {
    let user_api = Router::new()
        .route("/register", post(register_user))
        .route("/authenticate", post(authenticate_user))
        .route("/change_password", post(change_user_password));

    Router::new()
        .nest("/user", user_api)
        .with_state(app_state)

}
