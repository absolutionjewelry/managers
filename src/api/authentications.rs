use serde_json::Value;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use sha2::{Sha256, Digest};
use crate::{find_one_resource_where_fields, insert_resource, update_resource, delete_resource_where_fields};
use crate::models::authentication::{Authentication, AuthenticationError};
use crate::models::user::User;
use rocket::response::status;
use crate::api::token::{RawToken, VerifiedToken};
use rocket::http::Status;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    pub error: Option<AuthenticationError>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

impl AuthenticationResponse {
    pub fn success(data: Value, message: Option<String>) -> Self {
        Self { error: None, message: message, data: Some(data)}
    }

    pub fn error(error: AuthenticationError, message: String) -> Self {
        Self { error: Some(error), message: Some(message), data: None }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[post("/", data = "<authentication_request>")]
pub async fn login(authentication_request: Json<AuthenticationRequest>) -> status::Custom<Value> {
    let hashed_password = format!("{:x}", Sha256::digest(authentication_request.password.as_bytes()));

    let user = match find_one_resource_where_fields!(User, vec![("username", &authentication_request.username), ("user_password", &hashed_password)]).await {
        Ok(user) => user,
        Err(_) => return status::Custom(Status::NotFound, serde_json::to_value(AuthenticationResponse::error(AuthenticationError::UserNotFound, AuthenticationError::UserNotFound.to_string())).unwrap())
    };

    let user_id = user.id.unwrap();
    let expires_at = OffsetDateTime::now_utc() + time::Duration::days(30);
    let expires_at_str = expires_at.format(&Rfc3339).unwrap();

    match find_one_resource_where_fields!(Authentication, vec![("user_id", &user_id)]).await {
        Ok(authentication) =>
            match update_resource!(Authentication, authentication.id.clone(), vec![("user_id", &user_id), ("expires_at", &expires_at_str)]).await {
                Ok(_) => status::Custom(Status::Ok, serde_json::to_value(AuthenticationResponse::success(serde_json::to_value(authentication).unwrap(), None)).unwrap()),
                Err(_) => {
                    status::Custom(Status::BadRequest, serde_json::to_value(AuthenticationResponse::error(AuthenticationError::SessionUpdateFailed, AuthenticationError::SessionUpdateFailed.to_string())).unwrap())
                }
            }
        Err(_) => {
            let token = uuid::Uuid::new_v4().to_string();
            match insert_resource!(Authentication, vec![("user_id", &user_id), ("token", &token), ("expires_at", &expires_at_str)]).await {
                Ok(authentication) => status::Custom(Status::Ok, serde_json::to_value(AuthenticationResponse::success(serde_json::to_value(authentication).unwrap(), None)).unwrap()),
                Err(_) => {
                    status::Custom(Status::BadRequest, serde_json::to_value(AuthenticationResponse::error(AuthenticationError::SessionCreationFailed, AuthenticationError::SessionCreationFailed.to_string())).unwrap())
                }
            }
        }
    }
}

#[delete("/")]
pub async fn logout(token: RawToken) -> status::Custom<Value> {
    if token.value.is_empty() {
        return status::Custom(Status::BadRequest, serde_json::to_value(AuthenticationResponse::error(AuthenticationError::SessionNotFound, AuthenticationError::SessionNotFound.to_string())).unwrap())
    }
    let token_value = match VerifiedToken::from_raw(token).await {
        Ok(token) => token,
        Err(_) => return status::Custom(Status::BadRequest, serde_json::to_value(AuthenticationResponse::error(AuthenticationError::InvalidToken, AuthenticationError::InvalidToken.to_string())).unwrap())
    };
    let token_str = token_value.raw_token.unwrap().clone();
    match delete_resource_where_fields!(Authentication, vec![("token", &token_str)]).await {
        Ok(_) => status::Custom(Status::Ok, serde_json::to_value(AuthenticationResponse::success(serde_json::json!(null), Some("Logged out successfully".to_string()))).unwrap()),
        Err(_) => status::Custom(Status::BadRequest, serde_json::to_value(AuthenticationResponse::error(AuthenticationError::SessionNotFound, AuthenticationError::SessionNotFound.to_string())).unwrap())
    }
}

#[post("/register", data = "<register_request>")]
pub async fn register(register_request: Json<RegisterRequest>) -> status::Custom<Value> {
    let hashed_password = format!("{:x}", Sha256::digest(register_request.password.as_bytes()));

    match insert_resource!(User, vec![("username", &register_request.username), ("user_password", &hashed_password)]).await {
        Ok(user) => status::Custom(Status::Ok, serde_json::to_value(AuthenticationResponse::success(serde_json::to_value(user).unwrap(), None)).unwrap()),
        Err(err) => {
            println!("Error: {:?}", err);
            status::Custom(Status::BadRequest, serde_json::to_value(AuthenticationResponse::error(AuthenticationError::SessionNotFound, AuthenticationError::SessionNotFound.to_string())).unwrap())
        }
    }
}
