use crate::config::db_config::DbConfig;
use crate::db::repositories::user_repository::UserRepository;
use crate::services::crypto_service::{hash_str, verify_hash};
use sea_orm::DbErr;

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub async fn new(user_repository: UserRepository) -> Self {
        UserService { user_repository }
    }
    pub async fn from_config(config: DbConfig) -> Self {
        UserService::new(UserRepository::from_config(config).await).await
    }

    pub async fn new_user_access(&self, email: &str, application: &str, pwd: &str) -> Result<i64, String> {
        if pwd.len() < 8
            || !pwd.chars().any(|c| c.is_ascii_uppercase())
            || !pwd.chars().any(|c| c.is_ascii_lowercase())
            || !pwd.chars().any(|c| c.is_ascii_digit())
        {
            return Err("Password does not meet complexity requirements: at least 8 characters, including uppercase, lowercase, and digit.".to_string());
        }
        let user_id = if !(self.user_repository.user_exists(email).await.unwrap()) {
            self.user_repository.insert_user(email).await.unwrap()
        } else {
            self.user_repository
                .get_user_by_email(email)
                .await
                .unwrap()
                .unwrap()
                .id
        };
        if self
            .user_repository
            .access_exists(user_id, application)
            .await
            .unwrap()
        {
            return Err(format!("User {} already exists", email));
        }
        Ok(self
            .user_repository
            .insert_access(user_id, application, &hash_str(pwd))
            .await
            .unwrap())
    }

    async fn fake_authentication(&self, pwd: &str, with_fake_access_query: bool) -> bool {
        if with_fake_access_query {
            let _ = self
                .user_repository
                .get_access_by_user_id_and_application(-1, "123")
                .await;
        }
        hash_str(pwd);
        false
    }

    pub async fn authenticate_user(
        &self,
        email: &str,
        application: &str,
        pwd: &str,
    ) -> Result<bool, DbErr> {
        match self
            .user_repository
            .get_user_by_email(&email.to_string())
            .await
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
                    {
                        Ok(Some(access)) => Ok(verify_hash(&pwd, &access.pwd_hash)),
                        _ => Ok(self.fake_authentication(pwd, false).await),
                    }
                }
            },
            _ => Ok(self.fake_authentication(pwd, true).await),
        }
    }
}
