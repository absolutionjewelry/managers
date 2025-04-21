use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, postgres::PgValueRef, Decode, Error, Row, Type};
use std::str::FromStr;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
pub enum StoreGalleryError {
    StoreGalleryNotFound,
    StoreGalleryCreationFailed,
    StoreGalleryUpdateFailed,
    StoreGalleryDeletionFailed,
    InvalidGalleryType,
}

impl std::fmt::Display for StoreGalleryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreGalleryError::StoreGalleryNotFound => write!(f, "Store gallery not found"),
            StoreGalleryError::StoreGalleryCreationFailed => {
                write!(f, "Store gallery creation failed")
            }
            StoreGalleryError::StoreGalleryUpdateFailed => write!(f, "Store gallery update failed"),
            StoreGalleryError::StoreGalleryDeletionFailed => {
                write!(f, "Store gallery deletion failed")
            }
            StoreGalleryError::InvalidGalleryType => write!(f, "Invalid gallery type"),
        }
    }
}

impl std::error::Error for StoreGalleryError {}

#[derive(Debug, Deserialize, Serialize)]
pub enum StoreGalleryType {
    Store,
    Product,
}

impl std::fmt::Display for StoreGalleryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreGalleryType::Store => write!(f, "store"),
            StoreGalleryType::Product => write!(f, "product"),
        }
    }
}

impl FromStr for StoreGalleryType {
    type Err = StoreGalleryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "store" => Ok(StoreGalleryType::Store),
            "product" => Ok(StoreGalleryType::Product),
            _ => Err(StoreGalleryError::InvalidGalleryType),
        }
    }
}

impl<'r> Decode<'r, sqlx::Postgres> for StoreGalleryType {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<sqlx::Postgres>>::decode(value)?;
        Ok(match s {
            "store" => StoreGalleryType::Store,
            "product" => StoreGalleryType::Product,
            _ => return Err("invalid gallery type".into()),
        })
    }
}

impl Type<sqlx::Postgres> for StoreGalleryType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("text")
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreGallery {
    pub id: Option<String>,
    pub store_id: Option<String>,
    pub gallery_type: Option<StoreGalleryType>,
    pub gallery_type_id: Option<String>,
    pub gallery_name: Option<String>,
    pub gallery_description: Option<String>,

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

impl DatabaseResource for StoreGallery {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(StoreGallery {
            id: row.try_get("id")?,
            store_id: row.try_get("store_id")?,
            gallery_type: row.try_get("gallery_type")?,
            gallery_type_id: row.try_get("gallery_type_id")?,
            gallery_name: row.try_get("gallery_name")?,
            gallery_description: row.try_get("gallery_description")?,
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
