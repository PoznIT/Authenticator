use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterAccessRequest {
    pub application: String,
    pub pwd: String,
}

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub access: RegisterAccessRequest,
}

#[derive(Deserialize)]
pub struct AuthenticateUserRequest {
    pub email: String,
    pub application: String,
    pub pwd: String,
}

#[derive(Deserialize)]
pub struct UpdateUserPasswordRequest {
    pub email: String,
    pub application: String,
    pub old_pwd: String,
    pub new_pwd: String,
}
