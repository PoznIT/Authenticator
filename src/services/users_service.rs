use crate::config::db_config::DbConfig;
use crate::entities::{accesses, users};
use crate::services::crypto_service::{hash_str, verify_hash};
use sea_orm::{ActiveModelTrait, DbConn, QueryFilter};
use sea_orm::{ColumnTrait, EntityTrait};
use sea_orm::{DatabaseConnection, DbErr};

pub struct UserService {
    db_conn: DatabaseConnection,
}

impl UserService {
    pub async fn new(db_conn: DatabaseConnection) -> Self {
        UserService { db_conn }
    }
    pub async fn from_config(config: DbConfig) -> Self {
        UserService::new(config.get_db_connection().await).await
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email.to_owned()))
            .one(&self.db_conn)
            .await
    }

    async fn user_exists(&self, email: &str) -> Result<bool, DbErr> {
        Ok(self.get_user_by_email(email).await?.is_some())
    }

    async fn get_access_by_user_id_and_application(
        &self,
        user_id: i64,
        application: &str,
    ) -> Result<Option<accesses::Model>, DbErr> {
        accesses::Entity::find()
            .filter(accesses::Column::UserId.eq(user_id))
            .filter(accesses::Column::Application.eq(application.to_owned()))
            .one(&self.db_conn)
            .await
    }

    pub async fn add_user(&self, email: &str) -> Result<i64, DbErr> {
        let inserted_user = users::ActiveModel {
            email: sea_orm::ActiveValue::Set(email.to_string()),
            ..Default::default()
        };
        Ok(inserted_user.insert(&self.db_conn).await?.id)
    }

    pub async fn add_access(
        &self,
        user_id: i64,
        application: &str,
        pwd: &str,
    ) -> Result<i64, DbErr> {
        Ok(accesses::ActiveModel {
            user_id: sea_orm::ActiveValue::Set(user_id),
            application: sea_orm::ActiveValue::Set(application.to_string()),
            pwd_hash: sea_orm::ActiveValue::Set(pwd.to_string()),
            ..Default::default()
        }
        .insert(&self.db_conn)
        .await?
        .id)
    }

    pub async fn new_user(&self, email: &str, application: &str, pwd: &str) -> Result<i64, String> {
        if pwd.len() < 8
            || !pwd.chars().any(|c| c.is_ascii_uppercase())
            || !pwd.chars().any(|c| c.is_ascii_lowercase())
            || !pwd.chars().any(|c| c.is_ascii_digit())
        {
            return Err("Password does not meet complexity requirements: at least 8 characters, including uppercase, lowercase, and digit.".to_string());
        }
        if self.user_exists(email).await.unwrap() {
            return Err("User with this email already exists.".to_string());
        }
        let user_id = self.add_user(email).await.unwrap();
        let access_id = self
            .add_access(user_id, application, &hash_str(pwd))
            .await
            .unwrap();
        Ok(access_id)
    }

    async fn fake_authentication(&self, pwd: &str, with_fake_access_query: bool) -> bool {
        if with_fake_access_query {
            let _ = self.get_access_by_user_id_and_application(-1, "123").await;
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
        match self.get_user_by_email(&email.to_string()).await {
            Ok(user) => match user {
                None => Ok(self.fake_authentication(pwd, true).await),
                user => {
                    match self
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
