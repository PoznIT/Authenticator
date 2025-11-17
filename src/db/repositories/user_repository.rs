use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::{ColumnTrait, EntityTrait};
use sea_orm::{DatabaseConnection, DbErr};
use crate::config::db_config::DbConfig;
use crate::entities::{accesses, users};

pub struct UserRepository {
    db_conn: DatabaseConnection
}

impl UserRepository {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        UserRepository { db_conn }
    }
    pub async fn from_config(config: DbConfig) -> Self {
        UserRepository::new(config.get_db_connection().await)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email.to_owned()))
            .one(&self.db_conn)
            .await
    }

    pub async fn user_exists(&self, email: &str) -> Result<bool, DbErr> {
        Ok(self.get_user_by_email(email).await?.is_some())
    }

    pub async fn get_access_by_user_id_and_application(
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
    
    pub async fn access_exists(&self, user_id: i64, application: &str) -> Result<bool, DbErr> {
        Ok(self
            .get_access_by_user_id_and_application(user_id, application)
            .await?
            .is_some())
    }

    pub async fn insert_user(&self, email: &str) -> Result<i64, DbErr> {
        let inserted_user = users::ActiveModel {
            email: sea_orm::ActiveValue::Set(email.to_string()),
            ..Default::default()
        };
        Ok(inserted_user.insert(&self.db_conn).await?.id)
    }

    pub async fn insert_access(
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

    pub async fn delete_user(&self, email: &str) -> Result<(), DbErr> {
        if let Some(user) = self.get_user_by_email(email).await? {
            users::Entity::delete_by_id(user.id).exec(&self.db_conn).await?;
        }
        Ok(())
    }

    pub async fn delete_access(&self, user_id: i64) -> Result<(), DbErr> {
        accesses::Entity::delete_many()
            .filter(accesses::Column::UserId.eq(user_id))
            .exec(&self.db_conn)
            .await?;
        Ok(())
    }
}