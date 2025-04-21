use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
pub enum StoreImageError {
    StoreImageNotFound,
    StoreImageCreationFailed,
    StoreImageUpdateFailed,
    StoreImageDeletionFailed,
}

impl std::fmt::Display for StoreImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreImageError::StoreImageNotFound => write!(f, "Store image not found"),
            StoreImageError::StoreImageCreationFailed => write!(f, "Store image creation failed"),
            StoreImageError::StoreImageUpdateFailed => write!(f, "Store image update failed"),
            StoreImageError::StoreImageDeletionFailed => write!(f, "Store image deletion failed"),
        }
    }
}

impl std::error::Error for StoreImageError {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreImage {
    pub id: Option<String>,
    pub store_id: Option<String>,
    pub image_content_type: Option<String>,
    pub image_content: Option<Vec<u8>>,
    pub image_name: Option<String>,
    pub image_description: Option<String>,

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

impl DatabaseResource for StoreImage {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(StoreImage {
            id: row.try_get("id")?,
            store_id: row.try_get("store_id")?,
            image_content_type: row.try_get("image_content_type")?,
            image_content: row.try_get("image_content")?,
            image_name: row.try_get("image_name")?,
            image_description: row.try_get("image_description")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            archived_at: row.try_get("archived_at")?,
        })
    }

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
}
