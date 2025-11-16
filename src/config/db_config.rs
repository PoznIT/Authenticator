use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    db_name: String,
}

impl DbConfig {
    pub fn get_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }

    pub async fn get_db_connection(&self) -> DatabaseConnection {
        match Database::connect(&self.get_url()).await
        {
            Ok(conn) => conn,
            Err(e) => panic!("Failed to connect to database: {}", e),
        }
    }
}
