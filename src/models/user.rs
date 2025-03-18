use sha2::{Sha256, Digest};
use sqlx::{Error, FromRow};
use time::OffsetDateTime;
use crate::database::connection::get_connection;

#[derive(FromRow)]
pub struct User {
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub admin_status: bool,
    pub created_at: Option<OffsetDateTime>,
    pub archived_at: Option<OffsetDateTime>,
}

impl User {
    pub fn new(id: Option<String>, first_name: Option<String>, last_name: Option<String>, username: Option<String>, password: Option<String>, admin_status: bool, created_at: Option<OffsetDateTime>, archived_at: Option<OffsetDateTime>) -> Self {
        Self { id, first_name, last_name, username, password, admin_status, created_at, archived_at }
    }

    pub async fn find_by_username_and_password(username: String, password: String) -> Result<Option<Self>, Error> {
        let pool = get_connection().await;
        let hashed_password = format!("{:x}", Sha256::digest(password.as_bytes()));
        let result = sqlx::query!("SELECT * FROM users WHERE username = $1 AND user_password = $2", username, hashed_password)
            .fetch_optional(&pool)
            .await;
        match result {
            Ok(Some(user)) => Ok(Some(User::new(
                Some(user.id),
                Some(user.first_name),
                Some(user.last_name),
                Some(user.username),
                Some(user.user_password),
                user.admin_status,
                user.created_at,
                user.archived_at
            ))),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}


