use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
pub enum StoreProductError {
    StoreProductNotFound,
    StoreProductCreationFailed,
    StoreProductUpdateFailed,
    StoreProductDeletionFailed,
}

impl std::fmt::Display for StoreProductError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreProductError::StoreProductNotFound => write!(f, "Store product not found"),
            StoreProductError::StoreProductCreationFailed => {
                write!(f, "Store product creation failed")
            }
            StoreProductError::StoreProductUpdateFailed => write!(f, "Store product update failed"),
            StoreProductError::StoreProductDeletionFailed => {
                write!(f, "Store product deletion failed")
            }
        }
    }
}

impl std::error::Error for StoreProductError {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreProduct {
    pub id: Option<String>,
    pub store_id: Option<String>,
    pub product_name: Option<String>,
    pub product_description: Option<String>,
    pub product_base_price: Option<f64>,
    pub product_base_cost: Option<f64>,
    pub product_base_quantity: Option<f64>,

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

impl DatabaseResource for StoreProduct {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(StoreProduct {
            id: row.get("id"),
            store_id: row.get("store_id"),
            product_name: row.get("product_name"),
            product_description: row.get("product_description"),
            product_base_price: row.get("product_base_price"),
            product_base_cost: row.get("product_base_cost"),
            product_base_quantity: row.get("product_base_quantity"),
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
