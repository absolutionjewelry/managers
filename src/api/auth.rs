use serde_json::Value;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use crate::models::auth::{Auth, AuthError};
use crate::models::user::User;
use crate::models::session::Session;
use rocket::response::status;
use crate::api::token::RawToken;
use rocket::http::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub error: Option<AuthError>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

impl SessionResponse {
    pub fn success(data: Value, message: Option<String>) -> Self {
        Self { error: None, message: message, data: Some(data)}
    }

    pub fn error(error: AuthError, message: String) -> Self {
        Self { error: Some(error), message: Some(message), data: None }
    }
}

#[post("/", data = "<auth>")]
pub async fn login(auth: Json<Auth>) -> status::Custom<Value> {
    let user = match User::find_by_username_and_password(auth.username.clone(), auth.password.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return status::Custom(Status::NotFound, serde_json::to_value(SessionResponse::error(AuthError::UserNotFound, "User not found".to_string())).unwrap()),
        Err(_) => return status::Custom(Status::BadRequest, serde_json::to_value(SessionResponse::error(AuthError::InvalidCredentials, "Invalid credentials".to_string())).unwrap())
    };

    let user_id = user.id.unwrap_or_default();
    match Session::find_by_user_id(user_id.clone()).await {
        Ok(session) => {
            match Session::update(session.id.clone(), user_id).await {
                Ok(_) => status::Custom(Status::Ok, serde_json::to_value(SessionResponse::success(serde_json::to_value(session).unwrap(), None)).unwrap()),
                Err(_) => status::Custom(Status::BadRequest, serde_json::to_value(SessionResponse::error(AuthError::SessionCreationFailed, "Session update failed".to_string())).unwrap())
            }
        }
        Err(_) => {
            match Session::create(user_id).await {
                Ok(session) => status::Custom(Status::Ok, serde_json::to_value(SessionResponse::success(serde_json::to_value(session).unwrap(), None)).unwrap()),
                Err(_) => status::Custom(Status::BadRequest, serde_json::to_value(SessionResponse::error(AuthError::SessionCreationFailed, "Session creation failed".to_string())).unwrap())
            }
        }
    }
}

#[delete("/logout")]
pub async fn logout(token: RawToken) -> status::Custom<Value> {
    match Session::delete(token.value).await {
        Ok(_) => status::Custom(Status::Ok, serde_json::to_value(SessionResponse::success(serde_json::json!(null), Some("Logged out successfully".to_string()))).unwrap()),
        Err(_) => status::Custom(Status::BadRequest, serde_json::to_value(SessionResponse::error(AuthError::SessionCreationFailed, "Failed to logout".to_string())).unwrap())
    }
}
