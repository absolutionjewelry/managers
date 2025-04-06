use crate::api::response::Response;
use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::store_role::{StoreRole, StoreRoleError};
use crate::models::store_role_user::{StoreRoleUser, StoreRoleUserError};
use crate::{
    delete_resource_where_fields, find_one_resource_where_fields, insert_resource,
    join_all_resources_where_fields_on,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};

/// Get all roles assigned to a specific user in a store
///
/// Retrieves a list of all roles that have been assigned to the specified user
/// within the given store context.
///
/// # Permissions Required
/// - Store Admin or
/// - Store Manager
///
/// # URL Parameters
/// - store_id: The unique identifier of the store
/// - user_id: The unique identifier of the user
///
/// # Example curl request:
/// ```bash
/// curl -X GET \
///   'http://localhost:8000/api/stores/{store_id}/users/{user_id}/roles' \
///   -H 'Authorization: Bearer your_access_token'
/// ```
///
/// # Success Response (200 OK):
/// ```json
/// {
///   "success": true,
///   "data": [
///     {
///       "id": "role_id",
///       "store_id": "store_id",
///       "name": "Manager",
///       "permissions": ["read", "write"]
///     }
///   ],
///   "message": "Store user roles fetched successfully"
/// }
/// ```
///
/// # Error Responses
/// - 401 Unauthorized: Invalid or missing authentication token
/// - 403 Forbidden: Insufficient permissions
/// - 500 Internal Server Error: Failed to fetch roles
#[get("/<store_id>/users/<user_id>/roles")]
pub async fn get_store_user_roles(
    store_id: String,
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

    match join_all_resources_where_fields_on!(
        StoreRole,
        StoreUser,
        vec![
            ("store_id", DatabaseValue::String(store_id.clone())),
            ("user_id", DatabaseValue::String(user_id.clone()))
        ]
    )
    .await
    {
        Ok(roles) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(roles),
                Some("Store user roles fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleError::StoreRolesFetchFailed),
                StoreRoleError::StoreRolesFetchFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Assign a role to a user in a store
///
/// Creates a new role assignment linking a specific role to a user within
/// the context of a store. The role must already exist in the store.
///
/// # Permissions Required
/// - Store Admin
///
/// # URL Parameters
/// - store_id: The unique identifier of the store
/// - user_id: The unique identifier of the user
///
/// # Request Body
/// ```json
/// {
///   "id": "role_id",      // The ID of the existing role to assign
///   "store_id": "store_id" // Must match the store_id in the URL
/// }
/// ```
///
/// # Example curl request:
/// ```bash
/// curl -X POST \
///   'http://localhost:8000/api/stores/{store_id}/users/{user_id}/roles' \
///   -H 'Authorization: Bearer your_access_token' \
///   -H 'Content-Type: application/json' \
///   -d '{
///     "id": "role_id",
///     "store_id": "store_id"
///   }'
/// ```
///
/// # Success Response (201 Created):
/// ```json
/// {
///   "success": true,
///   "data": {
///     "id": "role_id",
///     "store_id": "store_id",
///     "name": "Manager",
///     "permissions": ["read", "write"]
///   },
///   "message": "Store user role created successfully"
/// }
/// ```
///
/// # Error Responses
/// - 400 Bad Request: Invalid request body
/// - 401 Unauthorized: Invalid or missing authentication token
/// - 403 Forbidden: Insufficient permissions
/// - 404 Not Found: Role not found
/// - 409 Conflict: Role already assigned to user
/// - 500 Internal Server Error: Failed to create role assignment
#[post("/<store_id>/users/<user_id>/roles", data = "<store_role>")]
pub async fn create_store_user_role(
    store_id: String,
    user_id: String,
    store_role: Json<StoreRole>,
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

    let store_role_id = store_role.id.clone();
    let params = vec![("id", &store_role_id), ("store_id", &store_id)];
    let store_role = match find_one_resource_where_fields!(StoreRole, params).await {
        Ok(role) => role,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreRoleError::StoreRoleNotFound),
                    StoreRoleError::StoreRoleNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    };

    let insert_params = vec![
        ("store_id", DatabaseValue::String(store_id)),
        ("user_id", DatabaseValue::String(user_id)),
        ("role_id", DatabaseValue::String(store_role.id.clone())),
    ];
    match insert_resource!(StoreRoleUser, insert_params).await {
        Ok(_) => status::Custom(
            Status::Created,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_role),
                Some("Store user role created successfully".to_string()),
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

/// Remove a role assignment from a user in a store
///
/// Deletes the association between a role and a user within the context
/// of a specific store.
///
/// # Permissions Required
/// - Store Admin
///
/// # URL Parameters
/// - store_id: The unique identifier of the store
/// - user_id: The unique identifier of the user
/// - role_id: The unique identifier of the role to remove
///
/// # Example curl request:
/// ```bash
/// curl -X DELETE \
///   'http://localhost:8000/api/stores/{store_id}/users/{user_id}/roles/{role_id}' \
///   -H 'Authorization: Bearer your_access_token'
/// ```
///
/// # Success Response (200 OK):
/// ```json
/// {
///   "success": true,
///   "data": "role_id",
///   "message": "Store user role deleted successfully"
/// }
/// ```
///
/// # Error Responses
/// - 401 Unauthorized: Invalid or missing authentication token
/// - 403 Forbidden: Insufficient permissions
/// - 404 Not Found: Role assignment not found
/// - 500 Internal Server Error: Failed to delete role assignment
#[delete("/<store_id>/users/<user_id>/roles/<role_id>")]
pub async fn delete_store_user_role(
    store_id: String,
    user_id: String,
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

    match delete_resource_where_fields!(
        StoreRoleUser,
        vec![
            ("store_id", DatabaseValue::String(store_id.clone())),
            ("user_id", DatabaseValue::String(user_id.clone())),
            ("role_id", DatabaseValue::String(role_id.clone()))
        ]
    )
    .await
    {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(role_id.clone()),
                Some("Store user role deleted successfully".to_string()),
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
