use crate::config::db_config::DbConfig;
use crate::db::repositories::user_repository::UserRepository;
use crate::services::crypto_service::CryptoService;
use sea_orm::DbErr;
use std::fmt;

#[derive(Debug)]
pub enum UserServiceError {
    PasswordComplexityNotMet,
    InvalidPassword(String),
    UserAlreadyExists(String),
    AuthenticationFailed,
    DatabaseError(String),
    UserNotFound(String),
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserServiceError::PasswordComplexityNotMet => write!(f, "Password does not meet complexity requirements: at least 8 characters, including uppercase, lowercase, and digit."),
            UserServiceError::InvalidPassword(msg) => write!(f, "Invalid password: {}", msg),
            UserServiceError::UserAlreadyExists(email) => write!(f, "User already exists: {}", email),
            UserServiceError::AuthenticationFailed => write!(f, "Authentication failed"),
            UserServiceError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            _ => {
                write!(f, "Unknown error")
            }
        }
    }
}

impl std::error::Error for UserServiceError {}

pub struct UserService<'a> {
    user_repository: UserRepository,
    crypto_service: CryptoService<'a>,
}

impl<'a> UserService<'a> {
    pub async fn new(user_repository: UserRepository) -> Self {
        UserService {
            user_repository,
            crypto_service: CryptoService::new(),
        }
    }
    pub async fn from_config(config: DbConfig) -> Self {
        UserService::new(UserRepository::from_config(config).await).await
    }
    fn validate_pwd(&self, pwd: &str) -> Result<(), UserServiceError> {
        if pwd.len() >= 8
            && pwd.chars().any(|c| c.is_ascii_uppercase())
            && pwd.chars().any(|c| c.is_ascii_lowercase())
            && pwd.chars().any(|c| c.is_ascii_digit()) {
            return Err(UserServiceError::PasswordComplexityNotMet)
        }
        Ok(())
    }

    pub async fn new_user_access(
        &self,
        email: &str,
        application: &str,
        pwd: &str,
    ) -> Result<i64, UserServiceError> {
        self.validate_pwd(pwd)?;
        let hash_pwd = self.crypto_service.hash_str(pwd);
        let user = self
            .user_repository
            .get_or_insert(email)
            .await
            .map_err(|err| UserServiceError::DatabaseError(err.to_string()))?;
        if self
            .user_repository
            .access_exists(user.id, &application)
            .await
            .map_err(|err| UserServiceError::DatabaseError(err.to_string()))?
        {
            return Err(UserServiceError::UserAlreadyExists(email.to_string()));
        }
        self.user_repository
            .insert_access(user.id, &application, &hash_pwd)
            .await
            .map_err(|err| UserServiceError::DatabaseError(err.to_string()))
    }

    async fn update_user_pwd(
        &self,
        email: &str,
        application: &str,
        old_pwd: &str,
        new_pwd: &str,
    ) -> Result<(), UserServiceError> {
        self.validate_pwd(&new_pwd)?;
        if !self
            .authenticate_user(email, application, old_pwd)
            .await
            .map_err(|err| UserServiceError::DatabaseError(err.to_string()))?
        {
            return Err(UserServiceError::AuthenticationFailed);
        }
        let hash_new_pwd = self.crypto_service.hash_str(new_pwd);
        self
            .user_repository
            .update_access_pwd_by_user_mail_and_application(email, application, &hash_new_pwd)
            .await
            .map_err(|err| UserServiceError::DatabaseError(err.to_string()))?;
        Ok(())
    }

    async fn fake_authentication(&self, pwd: &str, with_fake_access_query: bool) -> bool {
        if with_fake_access_query {
            let _ = self
                .user_repository
                .get_access_by_user_id_and_application(-1, "123")
                .await;
        }
        self.crypto_service.hash_str(pwd);
        false
    }

    pub async fn authenticate_user(
        &self,
        email: &str,
        application: &str,
        pwd: &str,
    ) -> Result<bool, UserServiceError> {
        match self
            .user_repository
            .get_user_by_email(&email.to_string())
            .await
            .map_err(|err| UserServiceError::DatabaseError(err.to_string()))
        {
            Ok(user) => match user {
                None => Ok(self.fake_authentication(pwd, true).await),
                user => {
                    match self
                        .user_repository
                        .get_access_by_user_id_and_application(
                            user.unwrap().id,
                            &application.to_string(),
                        )
                        .await
                        .map_err(|err| UserServiceError::DatabaseError(err.to_string()))
                    {
                        Ok(Some(access)) => {
                            Ok(self.crypto_service.verify_hash(&pwd, &access.pwd_hash))
                        }
                        _ => Ok(self.fake_authentication(pwd, false).await),
                    }
                }
            },
            _ => Ok(self.fake_authentication(pwd, true).await),
        }
    }
}
