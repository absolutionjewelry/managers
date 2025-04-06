use crate::api::response::Response;
use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::store_role_user::{StoreRoleUser, StoreRoleUserError};
use crate::models::store_user::StoreUser;
use crate::{delete_resource_where_fields, insert_resource, join_all_resources_where_fields_on};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};

/// Get all users with a specific role in a store
///
/// Retrieves a list of all users who have been assigned a specific role within a store.
/// Requires authentication via a valid access token.
///
/// # Authorization
/// - Requires a valid Bearer token in the Authorization header
///
/// # URL Parameters
/// - `store_id`: The unique identifier of the store
/// - `role_id`: The unique identifier of the role
///
/// # Example Request
/// ```bash
/// curl -X GET \
///   'http://localhost:8000/api/stores/store_123/roles/role_456/users' \
///   -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIs...'
/// ```
///
/// # Success Response (200 OK)
/// ```json
/// {
///   "data": [
///     {
///       "id": "user_123",
///       "email": "user@example.com",
///       "name": "John Doe",
///       "created_at": "2024-03-20T10:00:00Z"
///     }
///   ],
///   "message": "Store users fetched successfully"
/// }
/// ```
///
/// # Error Responses
/// - 401 Unauthorized: Invalid or missing token
/// - 500 Internal Server Error: Failed to fetch users
#[get("/<store_id>/roles/<role_id>/users")]
pub async fn get_store_role_users(
    store_id: String,
    role_id: String,
    token: RawToken,
) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
        Ok(token) => token,
        Err(_) => {
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(AuthenticationError::InvalidToken),
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            )
        }
    };

    match join_all_resources_where_fields_on!(
        StoreUser,
        StoreRole,
        vec![
            ("store_id", DatabaseValue::String(store_id.clone())),
            ("role_id", DatabaseValue::String(role_id.clone()))
        ]
    )
    .await
    {
        Ok(store_users) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_users),
                Some("Store users fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleUserError::StoreRoleUsersFetchFailed),
                StoreRoleUserError::StoreRoleUsersFetchFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Assign a role to a user in a store
///
/// Associates a user with a specific role in a store. This creates a new role-user relationship
/// in the specified store context.
///
/// # Authorization
/// - Requires a valid Bearer token in the Authorization header
///
/// # URL Parameters
/// - `store_id`: The unique identifier of the store
/// - `role_id`: The unique identifier of the role to assign
///
/// # Request Body
/// ```json
/// {
///   "id": "user_123"  // The ID of the user to assign the role to
/// }
/// ```
///
/// # Example Request
/// ```bash
/// curl -X POST \
///   'http://localhost:8000/api/stores/store_123/roles/role_456/users' \
///   -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIs...' \
///   -H 'Content-Type: application/json' \
///   -d '{
///     "id": "user_123"
///   }'
/// ```
///
/// # Success Response (201 Created)
/// ```json
/// {
///   "data": {
///     "store_id": "store_123",
///     "role_id": "role_456",
///     "user_id": "user_123",
///     "created_at": "2024-03-20T10:00:00Z"
///   },
///   "message": "Store user added to role successfully"
/// }
/// ```
///
/// # Error Responses
/// - 401 Unauthorized: Invalid or missing token
/// - 500 Internal Server Error: Failed to create role-user association
#[post("/<store_id>/roles/<role_id>/users", data = "<store_user>")]
pub async fn create_store_role_user(
    store_id: String,
    role_id: String,
    store_user: Json<StoreUser>,
    token: RawToken,
) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
        Ok(token) => token,
        Err(_) => {
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(AuthenticationError::InvalidToken),
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            )
        }
    };

    let store_user = store_user.into_inner();

    match insert_resource!(
        StoreRoleUser,
        vec![
            ("store_id", DatabaseValue::String(store_id.clone())),
            ("role_id", DatabaseValue::String(role_id.clone())),
            ("user_id", DatabaseValue::String(store_user.id))
        ]
    )
    .await
    {
        Ok(store_user) => status::Custom(
            Status::Created,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_user),
                Some("Store user added to role successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleUserError::StoreRoleUserCreationFailed),
                StoreRoleUserError::StoreRoleUserCreationFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Remove a role from a user in a store
///
/// Removes the association between a user and a role within a specific store context.
///
/// # Authorization
/// - Requires a valid Bearer token in the Authorization header
///
/// # URL Parameters
/// - `store_id`: The unique identifier of the store
/// - `role_id`: The unique identifier of the role
/// - `user_id`: The unique identifier of the user
///
/// # Example Request
/// ```bash
/// curl -X DELETE \
///   'http://localhost:8000/api/stores/store_123/roles/role_456/users/user_789' \
///   -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiIs...'
/// ```
///
/// # Success Response (200 OK)
/// ```json
/// {
///   "data": "user_789",
///   "message": "Store user deleted successfully"
/// }
/// ```
///
/// # Error Responses
/// - 401 Unauthorized: Invalid or missing token
/// - 500 Internal Server Error: Failed to delete role-user association
#[delete("/<store_id>/roles/<role_id>/users/<user_id>")]
pub async fn delete_store_role_user(
    store_id: String,
    role_id: String,
    user_id: String,
    token: RawToken,
) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
        Ok(token) => token,
        Err(_) => {
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(AuthenticationError::InvalidToken),
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            )
        }
    };

    match delete_resource_where_fields!(
        StoreRoleUser,
        vec![
            ("store_id", DatabaseValue::String(store_id.clone())),
            ("role_id", DatabaseValue::String(role_id.clone())),
            ("user_id", DatabaseValue::String(user_id.clone()))
        ]
    )
    .await
    {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(user_id.clone()),
                Some("Store user deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleUserError::StoreRoleUserDeletionFailed),
                StoreRoleUserError::StoreRoleUserDeletionFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}
