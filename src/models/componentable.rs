use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgHasArrayType, postgres::PgRow, Decode, Encode, Error, Row, Type};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ComponentableType {
    Product,
    Variant,
}

impl<'r> Decode<'r, sqlx::Postgres> for ComponentableType {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<sqlx::Postgres>>::decode(value)?;
        Ok(match s {
            "product" => ComponentableType::Product,
            "variant" => ComponentableType::Variant,
            _ => return Err("invalid componentable type".into()),
        })
    }
}

impl Type<sqlx::Postgres> for ComponentableType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("componentable_type")
    }
}

impl Encode<'_, sqlx::Postgres> for ComponentableType {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let s = match self {
            ComponentableType::Product => "product",
            ComponentableType::Variant => "variant",
        };
        <&str as Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl PgHasArrayType for ComponentableType {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_componentable_type")
    }
}

impl std::fmt::Display for ComponentableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentableType::Product => write!(f, "product"),
            ComponentableType::Variant => write!(f, "variant"),
        }
    }
}

impl std::str::FromStr for ComponentableType {
    type Err = ComponentableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "product" => Ok(ComponentableType::Product),
            "variant" => Ok(ComponentableType::Variant),
            _ => Err(ComponentableError::InvalidComponentableType),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ComponentableError {
    ComponentableNotFound,
    ComponentableCreationFailed,
    ComponentableUpdateFailed,
    ComponentableDeletionFailed,
    InvalidComponentableType,
}

impl std::fmt::Display for ComponentableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentableError::ComponentableNotFound => write!(f, "Componentable not found"),
            ComponentableError::ComponentableCreationFailed => {
                write!(f, "Componentable creation failed")
            }
            ComponentableError::ComponentableUpdateFailed => {
                write!(f, "Componentable update failed")
            }
            ComponentableError::ComponentableDeletionFailed => {
                write!(f, "Componentable deletion failed")
            }
            ComponentableError::InvalidComponentableType => {
                write!(f, "Invalid componentable type")
            }
        }
    }
}

impl std::error::Error for ComponentableError {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Componentable {
    pub id: Option<String>,
    pub component_id: Option<String>,
    pub componentable_id: Option<String>,
    pub componentable_type: Option<ComponentableType>,

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
}

impl DatabaseResource for Componentable {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Componentable {
            id: row.get("id"),
            component_id: row.get("component_id"),
            componentable_id: row.get("componentable_id"),
            componentable_type: row.get("componentable_type"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
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
