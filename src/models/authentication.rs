use serde::{Deserialize, Serialize};
use anyhow::Result;
use sqlx::{Error as SqlxError, postgres::PgRow, Row};
use crate::database::traits::DatabaseResource;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthenticationError {
    UserNotFound,
    InvalidCredentials,
    SessionCreationFailed,
    SessionDeletionFailed,
    SessionUpdateFailed,
    SessionNotFound,
    InvalidToken,
    TokenExpired,
}

impl std::fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthenticationError::UserNotFound => write!(f, "User not found"),
            AuthenticationError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthenticationError::SessionCreationFailed => write!(f, "Failed to create session"),
            AuthenticationError::SessionDeletionFailed => write!(f, "Failed to delete session"),
            AuthenticationError::SessionUpdateFailed => write!(f, "Failed to update session"),
            AuthenticationError::SessionNotFound => write!(f, "Session not found"),
            AuthenticationError::InvalidToken => write!(f, "Invalid token"),
            AuthenticationError::TokenExpired => write!(f, "Token expired"),
        }
    }
}

impl std::error::Error for AuthenticationError {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authentication {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expires_at: Option<String>,
    #[serde(serialize_with = "serialize_offset_date_time", deserialize_with = "deserialize_offset_date_time")]
    pub created_at: Option<OffsetDateTime>,
}

impl DatabaseResource for Authentication {
    fn from_row(row: &PgRow) -> Result<Self, SqlxError> {
        Ok(Authentication { id: row.get("id"), user_id: row.get("user_id"), token: row.get("token"), expires_at: row.get("expires_at"), created_at: row.get("created_at") })
    }

    fn has_id() -> bool {
        true
    }
}

fn serialize_offset_date_time<S>(dt: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match dt {
        Some(dt) => serializer.serialize_str(&dt.format(&Rfc3339).unwrap()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_offset_date_time<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) => OffsetDateTime::parse(&s, &Rfc3339)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
