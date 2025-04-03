use crate::api::token::{RawToken, VerifiedToken};
use crate::database::values::DatabaseValue;
use crate::models::{
    authentication::{Authentication, AuthenticationError},
    backup_code::{BackupCode, BackupCodeError},
    user::{User, UserError},
};
use crate::utils::backup_codes::generate_backup_codes;
use crate::{
    delete_resource_where_fields, find_one_resource_where_fields, insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use time::{format_description::well_known::Iso8601, Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthenticationResponseError {
    User(UserError),
    BackupCode(BackupCodeError),
    Authentication(AuthenticationError),
}

impl From<UserError> for AuthenticationResponseError {
    fn from(error: UserError) -> Self {
        AuthenticationResponseError::User(error)
    }
}

impl From<BackupCodeError> for AuthenticationResponseError {
    fn from(error: BackupCodeError) -> Self {
        AuthenticationResponseError::BackupCode(error)
    }
}

impl From<AuthenticationError> for AuthenticationResponseError {
    fn from(error: AuthenticationError) -> Self {
        AuthenticationResponseError::Authentication(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    pub error: Option<AuthenticationResponseError>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

impl AuthenticationResponse {
    pub fn success(data: Value, message: Option<String>) -> Self {
        Self {
            error: None,
            message: message,
            data: Some(data),
        }
    }

    pub fn error(error: AuthenticationResponseError, message: String) -> Self {
        Self {
            error: Some(error),
            message: Some(message),
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    pub username: String,
    pub password: String,
}

/// Login to the system
///
/// Parameters:
/// - username: String
/// - password: String
///
/// Returns:
/// if success:
/// - status: 200
/// - Authentication json object
/// else:
/// - error: AuthenticationError
///
/// Example:
/// "curl -X POST http://localhost:8000/api/auth/ -H 'Content-Type: application/json' -d '{"username": "admin", "password": "admin"}'"
#[post("/", data = "<authentication_request>")]
pub async fn login(authentication_request: Json<AuthenticationRequest>) -> status::Custom<Value> {
    let hashed_password = format!(
        "{:x}",
        Sha256::digest(authentication_request.password.as_bytes())
    );

    let login_params = vec![
        (
            "username",
            DatabaseValue::String(authentication_request.username.clone()),
        ),
        ("user_password", DatabaseValue::String(hashed_password)),
    ];
    let user = match find_one_resource_where_fields!(User, login_params).await {
        Ok(user) => user,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(AuthenticationResponse::error(
                    AuthenticationError::UserNotFound.into(),
                    AuthenticationError::UserNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    };

    let user_id = user.id.unwrap();
    let auth_params = vec![("user_id", DatabaseValue::String(user_id.clone()))];
    match find_one_resource_where_fields!(Authentication, auth_params).await {
        Ok(authentication) => {
            let auth_id = authentication.id.clone();
            let auth_value = serde_json::to_value(authentication).unwrap();
            match update_resource!(
                Authentication,
                auth_id,
                vec![(
                    "expires_at",
                    DatabaseValue::DateTime(
                        (OffsetDateTime::now_utc() + Duration::days(30))
                            .format(&Iso8601::DEFAULT)
                            .unwrap()
                    )
                )]
            )
            .await
            {
                Ok(_) => status::Custom(
                    Status::Ok,
                    serde_json::to_value(AuthenticationResponse::success(auth_value, None))
                        .unwrap(),
                ),
                Err(err) => {
                    println!("Error: {:?}", err);
                    return status::Custom(
                        Status::InternalServerError,
                        serde_json::to_value(AuthenticationResponse::error(
                            AuthenticationError::SessionUpdateFailed.into(),
                            AuthenticationError::SessionUpdateFailed.to_string(),
                        ))
                        .unwrap(),
                    );
                }
            }
        }
        Err(_) => {
            let token = Uuid::new_v4().to_string();
            match insert_resource!(
                Authentication,
                vec![
                    ("user_id", DatabaseValue::String(user_id.clone())),
                    ("token", DatabaseValue::String(token))
                ]
            )
            .await
            {
                Ok(authentication) => status::Custom(
                    Status::Ok,
                    serde_json::to_value(AuthenticationResponse::success(
                        serde_json::to_value(authentication).unwrap(),
                        None,
                    ))
                    .unwrap(),
                ),
                Err(_) => {
                    return status::Custom(
                        Status::InternalServerError,
                        serde_json::to_value(AuthenticationResponse::error(
                            AuthenticationError::SessionCreationFailed.into(),
                            AuthenticationError::SessionCreationFailed.to_string(),
                        ))
                        .unwrap(),
                    )
                }
            }
        }
    }
}

/// Logout from the system
///
/// Parameters:
/// - token: String (obtained from authentication)
///
/// Returns:
/// if success:
/// - status: 200
/// else:
/// - error: AuthenticationError
///
/// Example:
/// "curl -X DELETE http://localhost:8000/api/auth/ -H 'Content-Type: application/json' -H 'Authorization: Bearer <token>'"
#[delete("/")]
pub async fn logout(token: RawToken) -> status::Custom<Value> {
    if token.value.is_empty() {
        return status::Custom(
            Status::BadRequest,
            serde_json::to_value(AuthenticationResponse::error(
                AuthenticationError::SessionNotFound.into(),
                AuthenticationError::SessionNotFound.to_string(),
            ))
            .unwrap(),
        );
    }
    let token_value = match VerifiedToken::from_raw(token).await {
        Ok(token) => token,
        Err(_) => {
            return status::Custom(
                Status::BadRequest,
                serde_json::to_value(AuthenticationResponse::error(
                    AuthenticationError::InvalidToken.into(),
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            )
        }
    };
    let token_str = token_value.raw_token.unwrap().clone();
    let logout_params = vec![("token", DatabaseValue::String(token_str))];
    match delete_resource_where_fields!(Authentication, logout_params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(AuthenticationResponse::success(
                serde_json::json!(null),
                Some("Logged out successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::BadRequest,
            serde_json::to_value(AuthenticationResponse::error(
                AuthenticationError::SessionNotFound.into(),
                AuthenticationError::SessionNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user: User,
    pub backup_codes: Vec<String>,
}

/// Register a new user
///
/// Parameters:
/// - username: String
/// - password: String
///
/// Returns:
/// if success:
/// - status: 200
/// - User json object
/// else:
/// - error: AuthenticationError
///
/// Example:
/// "curl -X POST http://localhost:8000/api/auth/register -H 'Content-Type: application/json' -d '{"username": "admin", "password": "admin"}'"
#[post("/register", data = "<register_request>")]
pub async fn register(register_request: Json<RegisterRequest>) -> status::Custom<Value> {
    let hashed_password = format!("{:x}", Sha256::digest(register_request.password.as_bytes()));

    let register_params = vec![
        (
            "username",
            DatabaseValue::String(register_request.username.clone()),
        ),
        ("user_password", DatabaseValue::String(hashed_password)),
        (
            "first_name",
            DatabaseValue::String(register_request.first_name.clone()),
        ),
        (
            "last_name",
            DatabaseValue::String(register_request.last_name.clone()),
        ),
    ];
    let user = match insert_resource!(User, register_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error: {:?}", err);
            return status::Custom(
                Status::BadRequest,
                serde_json::to_value(AuthenticationResponse::error(
                    UserError::UserCreationFailed.into(),
                    UserError::UserCreationFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    };
    let user_id = user.id.clone().unwrap();
    let backup_codes = generate_backup_codes().await;
    for code in backup_codes.clone() {
        let backup_code_params = vec![
            ("user_id", DatabaseValue::String(user_id.clone())),
            ("code", DatabaseValue::String(code)),
        ];
        match insert_resource!(BackupCode, backup_code_params).await {
            Ok(_) => (),
            Err(err) => {
                println!("Error: {:?}", err);
                return status::Custom(
                    Status::BadRequest,
                    serde_json::to_value(AuthenticationResponse::error(
                        BackupCodeError::CodeCreationFailed.into(),
                        BackupCodeError::CodeCreationFailed.to_string(),
                    ))
                    .unwrap(),
                );
            }
        }
    }
    let register_response = RegisterResponse {
        user: user,
        backup_codes: backup_codes,
    };
    let response = AuthenticationResponse::success(
        serde_json::to_value(register_response).unwrap(),
        Some("User created successfully".to_string()),
    );
    status::Custom(Status::Ok, serde_json::to_value(response).unwrap())
}
