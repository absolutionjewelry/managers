use crate::{
    database::traits::DatabaseResource,
    utils::time::{deserialize_offset_date_time, serialize_offset_date_time},
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub enum StoreError {
    StoreNotFound,
    StoreCreationFailed,
    StoreUpdateFailed,
    StoreDeletionFailed,
    StoreNameAlreadyExists,
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::StoreNotFound => write!(f, "Store not found"),
            StoreError::StoreCreationFailed => write!(f, "Store creation failed"),
            StoreError::StoreUpdateFailed => write!(f, "Store update failed"),
            StoreError::StoreDeletionFailed => write!(f, "Store deletion failed"),
            StoreError::StoreNameAlreadyExists => write!(f, "Store name already exists"),
        }
    }
}

impl std::error::Error for StoreError {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub id: String,
    pub owner_id: String,
    pub store_name: String,
    pub store_description: String,

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

impl DatabaseResource for Store {
    fn has_id() -> bool {
        true
    }

    fn is_archivable() -> bool {
        true
    }

    fn is_updatable() -> bool {
        true
    }

    fn is_creatable() -> bool {
        true
    }

    fn is_expirable() -> bool {
        false
    }

    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Store {
            id: row.get("id"),
            owner_id: row.get("owner_id"),
            store_name: row.get("store_name"),
            store_description: row.get("store_description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            archived_at: row.get("archived_at"),
        })
    }
}
