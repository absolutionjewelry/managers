use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug)]
pub enum StoreRoleUserError {
    StoreRoleUserCreationFailed,
    StoreRoleUserDeletionFailed,
    StoreRoleUsersFetchFailed,
}

impl Into<anyhow::Error> for StoreRoleUserError {
    fn into(self) -> anyhow::Error {
        anyhow::anyhow!(self.to_string())
    }
}

impl std::fmt::Display for StoreRoleUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreRoleUserError::StoreRoleUserCreationFailed => {
                write!(f, "Store role user creation failed")
            }
            StoreRoleUserError::StoreRoleUserDeletionFailed => {
                write!(f, "Store role user deletion failed")
            }
            StoreRoleUserError::StoreRoleUsersFetchFailed => {
                write!(f, "Store role users fetch failed")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreRoleUser {
    pub id: String,
    pub store_id: String,
    pub user_id: String,
    pub role_id: String,

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub created_at: Option<OffsetDateTime>,

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub updated_at: Option<OffsetDateTime>,

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub archived_at: Option<OffsetDateTime>,
}

impl DatabaseResource for StoreRoleUser {
    fn has_id() -> bool {
        true
    }

    fn is_archivable() -> bool {
        true
    }

    fn is_updatable() -> bool {
        true
    }

    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Self {
            id: row.get("id"),
            store_id: row.get("store_id"),
            user_id: row.get("user_id"),
            role_id: row.get("role_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            archived_at: row.get("archived_at"),
        })
    }
}
