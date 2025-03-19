use sqlx::{Error, postgres::PgRow, Row};
use time::OffsetDateTime;
use crate::database::traits::DatabaseResource;
use rocket::serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

impl DatabaseResource for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(User { id: row.get("id"), first_name: row.get("first_name"), last_name: row.get("last_name"), username: row.get("username"), password: row.get("user_password"), admin_status: row.get("admin_status"), created_at: row.get("created_at"), archived_at: row.get("archived_at") })
    }

    fn has_id() -> bool {
        true
    }

    fn is_archivable() -> bool {
        true
    }
}


