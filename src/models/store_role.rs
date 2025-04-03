use crate::{
    database::traits::DatabaseResource,
    utils::time::{deserialize_offset_date_time, serialize_offset_date_time},
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub enum StoreRoleError {
    StoreRoleNotFound,
    StoreRoleCreationFailed,
    StoreRoleUpdateFailed,
    StoreRoleDeletionFailed,
    StoreRoleFetchFailed,
    StoreRolesFetchFailed,
}

impl std::fmt::Display for StoreRoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreRoleError::StoreRoleNotFound => write!(f, "Store role not found"),
            StoreRoleError::StoreRoleCreationFailed => write!(f, "Store role creation failed"),
            StoreRoleError::StoreRoleUpdateFailed => write!(f, "Store role update failed"),
            StoreRoleError::StoreRoleDeletionFailed => write!(f, "Store role deletion failed"),
            StoreRoleError::StoreRoleFetchFailed => write!(f, "Store role fetch failed"),
            StoreRoleError::StoreRolesFetchFailed => write!(f, "Store roles fetch failed"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StoreRole {
    pub id: String,
    pub store_id: String,
    pub role_name: String,
    pub role_description: String,

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

impl DatabaseResource for StoreRole {
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
        Ok(StoreRole {
            id: row.get("id"),
            store_id: row.get("store_id"),
            role_name: row.get("role_name"),
            role_description: row.get("role_description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            archived_at: row.get("archived_at"),
        })
    }

    fn is_creatable() -> bool {
        true
    }

    fn is_expirable() -> bool {
        false
    }
}
