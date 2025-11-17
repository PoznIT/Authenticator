use crate::config::db_config::DbConfig;
use crate::entities::{accesses, users};
use sea_orm::JoinType::InnerJoin;
use sea_orm::{ActiveModelTrait, QueryFilter, QuerySelect, RelationTrait};
use sea_orm::{ColumnTrait, EntityTrait};
use sea_orm::{DatabaseConnection, DbErr};

#[derive(Clone)]
pub struct UserRepository {
    db_conn: DatabaseConnection
}

impl UserRepository {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        UserRepository { db_conn }
    }
    pub async fn from_config(config: DbConfig) -> Self {
        let db_conn = config.get_db_connection().await;
        UserRepository::new(db_conn)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db_conn)
            .await
    }

    pub async fn get_or_insert(&self, email: &str) -> Result<users::Model, DbErr> {
       if let Some(user) = self.get_user_by_email(email).await? {
           return Ok(user)
       }
        Ok(self.insert_user(email).await?)
    }

    pub async fn get_access_by_user_id_and_application(
        &self,
        user_id: i64,
        application: &str,
    ) -> Result<Option<accesses::Model>, DbErr> {
        accesses::Entity::find()
            .filter(accesses::Column::UserId.eq(user_id))
            .filter(accesses::Column::Application.eq(application))
            .one(&self.db_conn)
            .await
    }

    pub async fn get_access_by_user_email_and_application(
        &self,
        email: &str,
        application: &str,
    ) -> Result<Option<accesses::Model>, DbErr> {
        accesses::Entity::find()
            .join(InnerJoin, accesses::Relation::Users.def())
            .filter(accesses::Column::Application.eq(application))
            .filter(users::Column::Email.eq(email))
            .one(&self.db_conn)
            .await
    }
    
    pub async fn access_exists(&self, user_id: i64, application: &str) -> Result<bool, DbErr> {
        Ok(self
            .get_access_by_user_id_and_application(user_id, application)
            .await?
            .is_some())
    }

    pub async fn insert_user(&self, email: &str) -> Result<users::Model, DbErr> {
        let inserted_user = users::ActiveModel {
            email: sea_orm::ActiveValue::Set(email.to_string()),
            ..Default::default()
        };
        Ok(inserted_user.insert(&self.db_conn).await?)
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

    pub async fn update_access_pwd_by_user_mail_and_application(&self, user_mail: &str, application: &str, pwd_hash: &str) -> Result<(), DbErr> {
        let access = self.get_access_by_user_email_and_application(&user_mail, application).await?;
        if let Some(access) = access {
            let mut access_am: accesses::ActiveModel = access.into();
            access_am.pwd_hash = sea_orm::ActiveValue::Set(pwd_hash.to_string());
            access_am.update(&self.db_conn).await?;
        }
        Ok(())
    }
}
