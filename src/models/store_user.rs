use crate::{
    database::traits::DatabaseResource,
    utils::time::{deserialize_offset_date_time, serialize_offset_date_time},
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub enum StoreUserError {
    StoreUserNotFound,
    StoreUserCreationFailed,
    StoreUserUpdateFailed,
    StoreUserDeletionFailed,
    StoreUserFetchFailed,
    StoreUsersFetchFailed,
}

impl std::fmt::Display for StoreUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreUserError::StoreUserNotFound => write!(f, "Store user not found"),
            StoreUserError::StoreUserCreationFailed => write!(f, "Store user creation failed"),
            StoreUserError::StoreUserUpdateFailed => write!(f, "Store user update failed"),
            StoreUserError::StoreUserDeletionFailed => write!(f, "Store user deletion failed"),
            StoreUserError::StoreUserFetchFailed => write!(f, "Store user fetch failed"),
            StoreUserError::StoreUsersFetchFailed => write!(f, "Store users fetch failed"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoreUser {
    pub id: String,
    pub store_id: String,
    pub user_id: String,

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

impl DatabaseResource for StoreUser {
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
        Ok(StoreUser {
            id: row.get("id"),
            store_id: row.get("store_id"),
            user_id: row.get("user_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            archived_at: row.get("archived_at"),
        })
    }
}
