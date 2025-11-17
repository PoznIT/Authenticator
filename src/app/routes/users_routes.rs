use crate::app::app::AppState;
use crate::app::routes::users_dto::{AuthenticateUserRequest, RegisterUserRequest, UpdateUserPasswordRequest};
use crate::services::users_service::UserServiceError;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

impl IntoResponse for UserServiceError {
    fn into_response(self) -> axum::response::Response {
        match self {
            UserServiceError::PasswordComplexityNotMet => StatusCode::BAD_REQUEST.into_response(),
            UserServiceError::UserAlreadyExists(_) => StatusCode::CONFLICT.into_response(),
            UserServiceError::UserNotFound(_) => StatusCode::BAD_REQUEST.into_response(),
            UserServiceError::AccessNotFound => StatusCode::BAD_REQUEST.into_response(),
            UserServiceError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }}


async fn register_user(
    State(state): State<AppState>,
    Json(req): Json<RegisterUserRequest>,
) -> Result<(), StatusCode> {
    match state
        .user_service
        .new_user_access(&req.email, &req.access.application, &req.access.pwd)
        .await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into_response().status())
    }
}

async fn authenticate_user(
    State(state): State<AppState>,
    Json(req): Json<AuthenticateUserRequest>,
) -> Result<Json<bool>, StatusCode> {
    match state
        .user_service
        .authenticate_user(&req.email, &req.application, &req.pwd)
        .await {
        Ok(is_authenticated) => Ok(Json(is_authenticated)),
        Err(err) => Err(err.into_response().status()),
    }
}

async fn update_user_password(
    State(state): State<AppState>,
    Json(req): Json<UpdateUserPasswordRequest>,
) -> Result<(), StatusCode> {
    match state
        .user_service
        .update_user_pwd(&req.email, &req.application, &req.old_pwd, &req.new_pwd)
        .await {
        Ok(_) => Ok(()),
        Err(err) => return Err(err.into_response().status()),
    }
}

pub fn init_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_user))
        .route("/authenticate", post(authenticate_user))
        .route("/change_password", post(update_user_password))
}
