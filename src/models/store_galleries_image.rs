use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
pub enum StoreGalleriesImageError {
    StoreGalleriesImageNotFound,
    StoreGalleriesImageCreationFailed,
    StoreGalleriesImageUpdateFailed,
    StoreGalleriesImageDeletionFailed,
}

impl std::fmt::Display for StoreGalleriesImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreGalleriesImageError::StoreGalleriesImageNotFound => {
                write!(f, "Store gallery image not found")
            }
            StoreGalleriesImageError::StoreGalleriesImageCreationFailed => {
                write!(f, "Store gallery image creation failed")
            }
            StoreGalleriesImageError::StoreGalleriesImageUpdateFailed => {
                write!(f, "Store gallery image update failed")
            }
            StoreGalleriesImageError::StoreGalleriesImageDeletionFailed => {
                write!(f, "Store gallery image deletion failed")
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StoreGalleriesImage {
    pub id: Option<String>,
    pub store_id: Option<String>,
    pub store_gallery_id: Option<String>,
    pub store_gallery_type: Option<String>,
    pub store_gallery_position: Option<i32>,
    pub store_image_id: Option<String>,

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub created_at: Option<OffsetDateTime>,
}

impl DatabaseResource for StoreGalleriesImage {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(StoreGalleriesImage {
            id: row.try_get("id")?,
            store_id: row.try_get("store_id")?,
            store_gallery_id: row.try_get("store_gallery_id")?,
            store_gallery_type: row.try_get("store_gallery_type")?,
            store_gallery_position: row.try_get("store_gallery_position")?,
            store_image_id: row.try_get("store_image_id")?,
            created_at: row.try_get("created_at")?,
        })
    }

    fn has_id() -> bool {
        true
    }

    fn is_archivable() -> bool {
        false
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
