use crate::services::users_service::UserService;

#[derive(Clone)]
pub struct AppState {
    user_service: UserService,
}

pub fn run_app(user_service: UserService) {
    AppState { user_service };
    
    
}