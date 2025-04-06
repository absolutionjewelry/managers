use crate::api::response::Response;
use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::store::{Store, StoreError};
use crate::models::store_role::{StoreRole, StoreRoleError};
use crate::{
    delete_resource_where_fields, find_all_resources_where_fields, find_one_resource_where_fields,
    insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};

/// Get all roles for a store
///
/// Retrieves all roles associated with the specified store. The requesting user must be
/// the store owner.
///
/// # Authorization
/// Requires a valid bearer token from a store owner
///
/// # Path Parameters
/// - `store_id`: The unique identifier of the store
///
/// # Returns
/// - 200 OK: Successfully retrieved store roles
/// - 401 Unauthorized: Invalid or missing token
/// - 404 Not Found: Store not found
/// - 500 Internal Server Error: Failed to fetch roles
///
/// # Example curl request:
/// ```bash
/// curl -X GET \
///   'http://localhost:8000/api/stores/{store_id}/roles' \
///   -H 'Authorization: Bearer {your_access_token}'
/// ```
///
/// # Example success response:
/// ```json
/// {
///   "data": [
///     {
///       "id": "role_123",
///       "store_id": "store_123",
///       "role_name": "Manager",
///       "role_description": "Store manager role"
///     }
///   ],
///   "message": "Store roles fetched successfully"
/// }
/// ```
#[get("/<store_id>/roles")]
pub async fn get_store_roles(store_id: String, token: RawToken) -> status::Custom<Value> {
    let token = match validate_token(token).await {
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
    let user_id = token.user_id;

    let store_id = store_id.clone();
    let params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
    ];
    let _ = match find_one_resource_where_fields!(Store, params).await {
        Ok(store) => store,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreNotFound),
                    StoreError::StoreNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    match find_all_resources_where_fields!(StoreRole, params).await {
        Ok(store_roles) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_roles),
                Some("Store roles fetched successfully".to_string()),
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

/// Get a specific role from a store
///
/// Retrieves details of a specific role in the store. The requesting user must be
/// the store owner.
///
/// # Authorization
/// Requires a valid bearer token from a store owner
///
/// # Path Parameters
/// - `store_id`: The unique identifier of the store
/// - `role_id`: The unique identifier of the role
///
/// # Returns
/// - 200 OK: Successfully retrieved store role
/// - 401 Unauthorized: Invalid or missing token
/// - 404 Not Found: Store or role not found
///
/// # Example curl request:
/// ```bash
/// curl -X GET \
///   'http://localhost:8000/api/stores/{store_id}/roles/{role_id}' \
///   -H 'Authorization: Bearer {your_access_token}'
/// ```
///
/// # Example success response:
/// ```json
/// {
///   "data": {
///     "id": "role_123",
///     "store_id": "store_123",
///     "role_name": "Manager",
///     "role_description": "Store manager role"
///   },
///   "message": "Store role fetched successfully"
/// }
/// ```
#[get("/<store_id>/roles/<role_id>")]
pub async fn get_store_role(
    store_id: String,
    role_id: String,
    token: RawToken,
) -> status::Custom<Value> {
    let token = match validate_token(token).await {
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
    let user_id = token.user_id;

    let store_id = store_id.clone();
    let role_id = role_id.clone();
    let params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
    ];
    let _ = match find_one_resource_where_fields!(Store, params).await {
        Ok(store) => store,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreNotFound),
                    StoreError::StoreNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let params = vec![
        ("id", DatabaseValue::String(role_id)),
        ("store_id", DatabaseValue::String(store_id)),
    ];
    match find_one_resource_where_fields!(StoreRole, params).await {
        Ok(store_role) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_role),
                Some("Store role fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleError::StoreRoleNotFound),
                StoreRoleError::StoreRoleNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Create a new role in a store
///
/// Creates a new role with the specified details. The requesting user must be
/// the store owner.
///
/// # Authorization
/// Requires a valid bearer token from a store owner
///
/// # Path Parameters
/// - `store_id`: The unique identifier of the store
///
/// # Request Body
/// - `store_id`: The store identifier (must match path parameter)
/// - `role_name`: Name of the role
/// - `role_description`: Description of the role's responsibilities
///
/// # Returns
/// - 201 Created: Successfully created store role
/// - 401 Unauthorized: Invalid or missing token
/// - 404 Not Found: Store not found
/// - 500 Internal Server Error: Failed to create role
///
/// # Example curl request:
/// ```bash
/// curl -X POST \
///   'http://localhost:8000/api/stores/{store_id}/roles' \
///   -H 'Authorization: Bearer {your_access_token}' \
///   -H 'Content-Type: application/json' \
///   -d '{
///     "store_id": "store_123",
///     "role_name": "Manager",
///     "role_description": "Store manager role"
///   }'
/// ```
///
/// # Example success response:
/// ```json
/// {
///   "data": {
///     "id": "role_123",
///     "store_id": "store_123",
///     "role_name": "Manager",
///     "role_description": "Store manager role"
///   },
///   "message": "Store role created successfully"
/// }
/// ```
#[post("/<store_id>/roles", data = "<store_role>")]
pub async fn create_store_role(
    store_id: String,
    store_role: Json<StoreRole>,
    token: RawToken,
) -> status::Custom<Value> {
    let token = match validate_token(token).await {
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
    let user_id = token.user_id;

    let store_id = store_id.clone();
    let params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
    ];
    let _ = match find_one_resource_where_fields!(Store, params).await {
        Ok(store) => store,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreNotFound),
                    StoreError::StoreNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let params = vec![
        (
            "store_id",
            DatabaseValue::String(store_role.store_id.clone()),
        ),
        (
            "role_name",
            DatabaseValue::String(store_role.role_name.clone()),
        ),
        (
            "role_description",
            DatabaseValue::String(store_role.role_description.clone()),
        ),
    ];
    match insert_resource!(StoreRole, params).await {
        Ok(store_role) => status::Custom(
            Status::Created,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_role),
                Some("Store role created successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleError::StoreRoleCreationFailed),
                StoreRoleError::StoreRoleCreationFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Update an existing role in a store
///
/// Updates the details of an existing role. The requesting user must be
/// the store owner.
///
/// # Authorization
/// Requires a valid bearer token from a store owner
///
/// # Path Parameters
/// - `store_id`: The unique identifier of the store
/// - `role_id`: The unique identifier of the role to update
///
/// # Request Body
/// - `store_id`: The store identifier (must match path parameter)
/// - `role_name`: Updated name of the role
/// - `role_description`: Updated description of the role
///
/// # Returns
/// - 200 OK: Successfully updated store role
/// - 401 Unauthorized: Invalid or missing token
/// - 404 Not Found: Store or role not found
/// - 500 Internal Server Error: Failed to update role
///
/// # Example curl request:
/// ```bash
/// curl -X PUT \
///   'http://localhost:8000/api/stores/{store_id}/roles/{role_id}' \
///   -H 'Authorization: Bearer {your_access_token}' \
///   -H 'Content-Type: application/json' \
///   -d '{
///     "store_id": "store_123",
///     "role_name": "Senior Manager",
///     "role_description": "Updated store manager role"
///   }'
/// ```
///
/// # Example success response:
/// ```json
/// {
///   "data": {
///     "id": "role_123",
///     "store_id": "store_123",
///     "role_name": "Senior Manager",
///     "role_description": "Updated store manager role"
///   },
///   "message": "Store role updated successfully"
/// }
/// ```
#[put("/<store_id>/roles/<role_id>", data = "<store_role>")]
pub async fn update_store_role(
    store_id: String,
    role_id: String,
    store_role: Json<StoreRole>,
    token: RawToken,
) -> status::Custom<Value> {
    let token = match validate_token(token).await {
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
    let user_id = token.user_id;

    let store_id = store_id.clone();
    let role_id = role_id.clone();
    let params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
    ];
    let _ = match find_one_resource_where_fields!(Store, params).await {
        Ok(store) => store,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreNotFound),
                    StoreError::StoreNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    match update_resource!(
        StoreRole,
        role_id,
        vec![
            (
                "role_name",
                DatabaseValue::String(store_role.role_name.clone())
            ),
            (
                "role_description",
                DatabaseValue::String(store_role.role_description.clone())
            ),
        ]
    )
    .await
    {
        Ok(store_role) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_role),
                Some("Store role updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleError::StoreRoleUpdateFailed),
                StoreRoleError::StoreRoleUpdateFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Delete a role from a store
///
/// Removes a role from the store. The requesting user must be the store owner.
///
/// # Authorization
/// Requires a valid bearer token from a store owner
///
/// # Path Parameters
/// - `store_id`: The unique identifier of the store
/// - `role_id`: The unique identifier of the role to delete
///
/// # Returns
/// - 200 OK: Successfully deleted store role
/// - 401 Unauthorized: Invalid or missing token
/// - 404 Not Found: Store or role not found
/// - 500 Internal Server Error: Failed to delete role
///
/// # Example curl request:
/// ```bash
/// curl -X DELETE \
///   'http://localhost:8000/api/stores/{store_id}/roles/{role_id}' \
///   -H 'Authorization: Bearer {your_access_token}'
/// ```
///
/// # Example success response:
/// ```json
/// {
///   "data": "role_123",
///   "message": "Store role deleted successfully"
/// }
/// ```
#[delete("/<store_id>/roles/<role_id>")]
pub async fn delete_store_role(
    store_id: String,
    role_id: String,
    token: RawToken,
) -> status::Custom<Value> {
    let token = match validate_token(token).await {
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
    let user_id = token.user_id;

    let store_id = store_id.clone();
    let role_id = role_id.clone();
    let params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
    ];
    let _ = match find_one_resource_where_fields!(Store, params).await {
        Ok(store) => store,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreNotFound),
                    StoreError::StoreNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let params = vec![
        ("id", DatabaseValue::String(role_id.clone())),
        ("store_id", DatabaseValue::String(store_id.clone())),
    ];
    match find_one_resource_where_fields!(StoreRole, params).await {
        Ok(store_role) => store_role,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreRoleError::StoreRoleNotFound),
                    StoreRoleError::StoreRoleNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let role_id_clone = role_id.clone();
    let store_id_clone = store_id.clone();
    match delete_resource_where_fields!(
        StoreRole,
        vec![
            ("id", DatabaseValue::String(role_id_clone)),
            ("store_id", DatabaseValue::String(store_id_clone))
        ]
    )
    .await
    {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(role_id.clone()),
                Some("Store role deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleError::StoreRoleDeletionFailed),
                StoreRoleError::StoreRoleDeletionFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}
