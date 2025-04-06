use crate::api::token::{validate_token, RawToken};
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
/// Authenticates a user with their username and password. If the user already has an active session,
/// it will be extended. Otherwise, a new session will be created with a 30-day expiration.
///
/// # Request Body
/// ```json
/// {
///     "username": "string",     // The user's unique username
///     "password": "string"      // The user's password (will be hashed)
/// }
/// ```
///
/// # Returns
/// - Success (200 OK):
///   ```json
///   {
///     "error": null,
///     "message": null,
///     "data": {
///       "id": "uuid",           // The authentication session ID
///       "user_id": "uuid",      // The authenticated user's ID
///       "token": "string",      // Bearer token to use for authenticated requests
///       "created_at": "datetime", // When the session was created
///       "expires_at": "datetime"  // When the session will expire (30 days from now)
///     }
///   }
///   ```
/// - Error (404 Not Found):
///   - When username/password combination is invalid
///   - When user account doesn't exist
/// - Error (500 Internal Server Error):
///   - When session creation fails
///   - When session update fails
///
/// # Example
/// ```bash
/// # Basic login
/// curl -X POST 'http://localhost:8000/api/auth/' \
///   -H 'Content-Type: application/json' \
///   -d '{
///     "username": "johndoe",
///     "password": "secretpass123"
///   }'
/// ```
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordRequest {
    pub username: String,
    pub code: String,
    pub new_password: String,
}

#[post("/reset-password", data = "<reset_password_request>")]
pub async fn reset_password(
    reset_password_request: Json<ResetPasswordRequest>,
) -> status::Custom<Value> {
    let user_params = vec![(
        "username",
        DatabaseValue::String(reset_password_request.username.clone()),
    )];
    let user = match find_one_resource_where_fields!(User, user_params).await {
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
    let backup_code_params = vec![
        ("user_id", DatabaseValue::String(user_id.clone())),
        (
            "code",
            DatabaseValue::String(reset_password_request.code.clone()),
        ),
    ];
    let backup_code = match find_one_resource_where_fields!(BackupCode, backup_code_params).await {
        Ok(backup_code) => backup_code,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(AuthenticationResponse::error(
                    BackupCodeError::CodeNotFound.into(),
                    BackupCodeError::CodeNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    };
    if backup_code.used.unwrap() {
        return status::Custom(
            Status::BadRequest,
            serde_json::to_value(AuthenticationResponse::error(
                BackupCodeError::CodeAlreadyUsed.into(),
                BackupCodeError::CodeAlreadyUsed.to_string(),
            ))
            .unwrap(),
        );
    }
    let backup_code_id = backup_code.id.unwrap();

    let update_backup_code_params = vec![("used", DatabaseValue::Boolean(true.to_string()))];
    match update_resource!(BackupCode, backup_code_id, update_backup_code_params).await {
        Ok(_) => (),
        Err(_) => {
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(AuthenticationResponse::error(
                    BackupCodeError::CodeUpdateFailed.into(),
                    BackupCodeError::CodeUpdateFailed.to_string(),
                ))
                .unwrap(),
            )
        }
    };

    let hashed_password = format!(
        "{:x}",
        Sha256::digest(reset_password_request.new_password.as_bytes())
    );
    let update_params = vec![("user_password", DatabaseValue::String(hashed_password))];
    match update_resource!(User, user_id, update_params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(AuthenticationResponse::success(
                serde_json::json!(null),
                Some("Password reset successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(AuthenticationResponse::error(
                UserError::UserUpdateFailed.into(),
                UserError::UserUpdateFailed.to_string(),
            ))
            .unwrap(),
        ),
    };
    login(Json(AuthenticationRequest {
        username: reset_password_request.username.clone(),
        password: reset_password_request.new_password.clone(),
    }))
    .await
}

/// Logout from the system
///
/// Invalidates the current user session by deleting their authentication token.
/// After logout, the token can no longer be used for authenticated requests.
///
/// # Headers Required
/// - Authorization: Bearer <token>
///   - The token must be a valid authentication token obtained from login
///   - The token must not be expired
///   - The token must be prefixed with "Bearer "
///
/// # Returns
/// - Success (200 OK):
///   ```json
///   {
///     "error": null,
///     "message": "Logged out successfully",
///     "data": null
///   }
///   ```
/// - Error (400 Bad Request):
///   - When the token is missing
///   - When the token format is invalid
///   - When the token has already been invalidated
///   - When the session is not found
///
/// # Example
/// ```bash
/// # Logout with a valid token
/// curl -X DELETE 'http://localhost:8000/api/auth/' \
///   -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIs...'
///
/// # Note: Replace the token with your actual authentication token
/// ```
#[delete("/")]
pub async fn logout(token: RawToken) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
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
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponse {
    pub user: User,
    pub backup_codes: Vec<String>,
}

/// Register a new user
///
/// Creates a new user account and generates backup codes for account recovery.
/// The backup codes should be stored securely by the user as they can be used
/// to recover account access if the password is lost.
///
/// # Request Body
/// ```json
/// {
///     "username": "string",     // Unique username for the account
///     "password": "string",     // Password (will be hashed before storage)
///     "firstName": "string",    // User's first name
///     "lastName": "string"      // User's last name
/// }
/// ```
///
/// # Returns
/// - Success (200 OK):
///   ```json
///   {
///     "error": null,
///     "message": "User created successfully",
///     "data": {
///       "user": {
///         "id": "uuid",           // Unique identifier for the user
///         "username": "string",   // The registered username
///         "firstName": "string",  // User's first name
///         "lastName": "string",   // User's last name
///         "createdAt": "datetime" // When the account was created
///       },
///       "backupCodes": [         // One-time use backup codes for account recovery
///         "string",              // Store these securely
///         "string",
///         ...
///       ]
///     }
///   }
///   ```
/// - Error (400 Bad Request):
///   - When username is already taken
///   - When required fields are missing
///   - When user creation fails
///   - When backup code generation fails
///
/// # Security Notes
/// - Passwords are hashed using SHA-256 before storage
/// - Backup codes are generated randomly and should be stored securely
/// - Each backup code can only be used once for account recovery
///
/// # Example
/// ```bash
/// # Register a new user
/// curl -X POST 'http://localhost:8000/api/auth/register' \
///   -H 'Content-Type: application/json' \
///   -d '{
///     "username": "johndoe",
///     "password": "secretpass123",
///     "firstName": "John",
///     "lastName": "Doe"
///   }'
/// ```
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
