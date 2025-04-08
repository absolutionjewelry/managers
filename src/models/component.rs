use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
pub enum ComponentError {
    ComponentNotFound,
    ComponentCreationFailed,
    ComponentUpdateFailed,
    ComponentDeletionFailed,
}

impl std::fmt::Display for ComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentError::ComponentNotFound => write!(f, "Component not found"),
            ComponentError::ComponentCreationFailed => write!(f, "Component creation failed"),
            ComponentError::ComponentUpdateFailed => write!(f, "Component update failed"),
            ComponentError::ComponentDeletionFailed => write!(f, "Component deletion failed"),
        }
    }
}

impl std::error::Error for ComponentError {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub id: Option<String>,
    pub component_name: Option<String>,
    pub component_description: Option<String>,
    pub cost_per_unit: Option<f64>,
    pub quantity: Option<f64>,
    pub unit_of_measure: Option<String>,

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

impl DatabaseResource for Component {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Component {
            id: row.get("id"),
            component_name: row.get("component_name"),
            component_description: row.get("component_description"),
            cost_per_unit: row.get("cost_per_unit"),
            quantity: row.get("quantity"),
            unit_of_measure: row.get("unit_of_measure"),
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
