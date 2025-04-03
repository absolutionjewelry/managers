use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::{postgres::PgRow, Row};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VariantError {
    VariantNotFound,
    VariantCreationFailed,
    VariantUpdateFailed,
    VariantDeletionFailed,
}

impl std::fmt::Display for VariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariantError::VariantNotFound => write!(f, "Variant not found"),
            VariantError::VariantCreationFailed => write!(f, "Variant creation failed"),
            VariantError::VariantUpdateFailed => write!(f, "Variant update failed"),
            VariantError::VariantDeletionFailed => write!(f, "Variant deletion failed"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub id: Option<String>,

    #[serde(rename = "storeId")]
    pub store_id: Option<String>,

    #[serde(rename = "variantName")]
    pub variant_name: Option<String>,

    #[serde(rename = "variantDescription")]
    pub variant_description: Option<String>,

    #[serde(rename = "variantBaseCost")]
    pub variant_base_cost: Option<f64>,

    #[serde(rename = "variantBasePrice")]
    pub variant_base_price: Option<f64>,

    #[serde(rename = "variantBaseQuantity")]
    pub variant_base_quantity: Option<i32>,

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

impl DatabaseResource for Variant {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Variant {
            id: row.get("id"),
            store_id: row.get("store_id"),
            variant_name: row.get("variant_name"),
            variant_description: row.get("variant_description"),
            variant_base_cost: row.get("variant_base_cost"),
            variant_base_price: row.get("variant_base_price"),
            variant_base_quantity: row.get("variant_base_quantity"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            archived_at: row.get("archived_at"),
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
