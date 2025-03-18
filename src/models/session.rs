use serde::{Deserialize, Serialize};
use sqlx::Error;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use crate::database::connection::get_connection;
use crate::utils::time::serialize_offset_date_time;
use crate::utils::time::deserialize_offset_date_time;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Session {
    pub id: String,
    pub user_id: String,
    #[serde(serialize_with = "serialize_offset_date_time", deserialize_with = "deserialize_offset_date_time")]
    pub expires_at: Option<OffsetDateTime>,
}

impl Session {
    pub fn new(id: String, user_id: String, expires_at: Option<OffsetDateTime>) -> Self {
        Self { id, user_id, expires_at }
    }

    pub async fn find_by_id(id: String) -> Result<Self, Error> {
        let pool = get_connection().await;
        match sqlx::query!("SELECT * FROM auth WHERE id = $1", id)
            .fetch_one(&pool)
            .await {
                Ok(session) => Ok(Session::new(
                    session.id,
                    session.user_id,
                    Some(session.expires_at)
                )),
                Err(e) => Err(e),
            }
    }

    pub async fn find_by_user_id(user_id: String) -> Result<Self, Error> {
        let pool = get_connection().await;
        match sqlx::query!("SELECT * FROM auth WHERE user_id = $1", user_id)
            .fetch_one(&pool)
            .await {
                Ok(session) => Ok(Session::new(
                    session.id,
                    session.user_id,
                    Some(session.expires_at)
                )),
                Err(e) => Err(e),
            }
    }

    pub async fn create(user_id: String) -> Result<Self, Error> {
        let pool = get_connection().await;
        let dt = OffsetDateTime::now_utc() + time::Duration::days(30);
        let expires_at = PrimitiveDateTime::new(dt.date(), dt.time()).assume_utc();
        let id = Uuid::new_v4().to_string();
        let session = sqlx::query!("INSERT INTO auth (id, user_id, expires_at) VALUES ($1, $2, $3) RETURNING *", id, user_id, expires_at)
            .fetch_one(&pool)
            .await;
        match session {
            Ok(session) => Ok(Session::new(
                session.id,
                session.user_id,
                Some(session.expires_at)
            )),
            Err(e) => Err(e),
        }
    }

    pub async fn update(id: String, user_id: String) -> Result<Session, Error> {
        let pool = get_connection().await;
        let dt = OffsetDateTime::now_utc() + time::Duration::days(30);
        let expires_at = PrimitiveDateTime::new(dt.date(), dt.time()).assume_utc();
        match Session::find_by_id(id.clone()).await {
            Ok(session) => session,
            Err(e) => return Err(e),
        };
        match sqlx::query!("UPDATE auth SET expires_at = $1 WHERE id = $2 AND user_id = $3", expires_at, id, user_id)
            .execute(&pool)
            .await {
                Ok(_) => {
                    let session = match Session::find_by_id(id.clone()).await {
                        Ok(session) => session,
                        Err(e) => return Err(e),
                    };
                    Ok(session)
                },
                Err(e) => Err(e),
            }
    }

    pub async fn delete(id: String) -> Result<(), Error> {
        let pool = get_connection().await;
        match sqlx::query!("DELETE FROM auth WHERE id = $1", id)
            .execute(&pool)
            .await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
    }
}
