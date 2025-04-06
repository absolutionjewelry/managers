use crate::api::response::Response;
use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::store::{Store, StoreError};
use crate::models::store_user::{StoreUser, StoreUserError};
use crate::{
    delete_resource_where_fields, find_all_resources_where_fields, find_one_resource_where_fields,
    insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};

/// Get all users of a store
///
/// Retrieves all users associated with a specific store. Only the store owner can access this endpoint.
/// This endpoint is useful for managing store permissions and viewing all users who have access to the store.
///
/// Required headers:
/// - Authorization: Bearer token
///
/// Path parameters:
/// - store_id: The ID of the store
///
/// Response body:
/// ```json
/// {
///   "data": [
///     {
///       "id": "789",
///       "user_id": "456",
///       "store_id": "123"
///     },
///     // ... more users
///   ],
///   "message": "Store users fetched successfully"
/// }
/// ```
///
/// Returns:
/// - 200 OK: List of store users
/// - 401 Unauthorized: Invalid or missing token
/// - 404 Not Found: Store not found
/// - 500 Internal Server Error: Failed to fetch store users
///
/// Example:
/// ```bash
/// curl -X GET 'http://localhost:8000/api/stores/123/users' \
///   -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiJ9...'
/// ```
#[get("/<store_id>/users")]
pub async fn get_store_users(store_id: String, token: RawToken) -> status::Custom<Value> {
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

    let check_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
    ];
    let _ = match find_one_resource_where_fields!(Store, check_params).await {
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

    let query_params = vec![("store_id", DatabaseValue::String(store_id))];
    match find_all_resources_where_fields!(StoreUser, query_params).await {
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
                anyhow::anyhow!(StoreUserError::StoreUsersFetchFailed),
                StoreUserError::StoreUsersFetchFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Get a specific user of a store
///
/// Retrieves details of a specific user in a store. Only the store owner can access this endpoint.
///
/// Required headers:
/// - Authorization: Bearer token
///
/// Path parameters:
/// - store_id: The ID of the store
/// - user_id: The ID of the user to retrieve
///
/// Returns:
/// - 200 OK: Store user details
/// - 401 Unauthorized: Invalid or missing token
/// - 404 Not Found: Store or user not found
///
/// Example:
/// ```bash
/// curl -X GET 'http://localhost:8000/api/stores/123/users/456' \
///   -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiJ9...'
/// ```
#[get("/<store_id>/users/<user_id>")]
pub async fn get_store_user(
    store_id: String,
    user_id: String,
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
    let owner_id = token.user_id;

    let check_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(owner_id.clone())),
    ];
    let _ = match find_one_resource_where_fields!(Store, check_params).await {
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

    let query_params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("user_id", DatabaseValue::String(user_id.clone())),
    ];
    match find_one_resource_where_fields!(StoreUser, query_params).await {
        Ok(store_user) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_user),
                Some("Store user fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreUserError::StoreUserNotFound),
                StoreUserError::StoreUserNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Create a new store user
///
/// Adds a new user to a store. Only the store owner can access this endpoint.
/// This endpoint allows store owners to grant access to their store for other users.
/// The user must exist in the system before they can be added to a store.
///
/// Required headers:
/// - Authorization: Bearer token
/// - Content-Type: application/json
///
/// Path parameters:
/// - store_id: The ID of the store
///
/// Request body:
/// ```json
/// {
///   "user_id": "456",
///   "store_id": "123"
/// }
/// ```
///
/// Response body:
/// ```json
/// {
///   "data": {
///     "id": "789",
///     "user_id": "456",
///     "store_id": "123"
///   },
///   "message": "Store user created successfully"
/// }
/// ```
///
/// Returns:
/// - 201 Created: Store user created successfully
/// - 401 Unauthorized: Invalid or missing token
/// - 404 Not Found: Store not found or user not found
/// - 409 Conflict: User already exists in store
/// - 500 Internal Server Error: Failed to create store user
///
/// Example:
/// ```bash
/// curl -X POST 'http://localhost:8000/api/stores/123/users' \
///   -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiJ9...' \
///   -H 'Content-Type: application/json' \
///   -d '{"user_id": "456", "store_id": "123"}'
/// ```
#[post("/<store_id>/users", data = "<store_user>")]
pub async fn create_store_user(
    store_id: String,
    store_user: Json<StoreUser>,
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

    let owner_id = token.user_id;

    let check_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(owner_id)),
    ];
    let _ = match find_one_resource_where_fields!(Store, check_params).await {
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

    let store_user = store_user.into_inner();

    let store_id_clone = store_id.clone();
    match insert_resource!(
        StoreUser,
        vec![
            ("store_id", DatabaseValue::String(store_id_clone)),
            ("user_id", DatabaseValue::String(store_user.user_id))
        ]
    )
    .await
    {
        Ok(store_user) => status::Custom(
            Status::Created,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_user),
                Some("Store user created successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreUserError::StoreUserCreationFailed),
                StoreUserError::StoreUserCreationFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Update a store user
///
/// Updates details of a specific user in a store. Only the store owner can access this endpoint.
///
/// Required headers:
/// - Authorization: Bearer token
/// - Content-Type: application/json
///
/// Path parameters:
/// - store_id: The ID of the store
/// - user_id: The ID of the user to update
///
/// Request body:
/// ```json
/// {
///   "id": "789",
///   "user_id": "456",
///   "store_id": "123"
/// }
/// ```
///
/// Returns:
/// - 200 OK: Store user updated successfully
/// - 401 Unauthorized: Invalid or missing token
/// - 404 Not Found: Store or user not found
/// - 500 Internal Server Error: Failed to update store user
///
/// Example:
/// ```bash
/// curl -X PUT 'http://localhost:8000/api/stores/123/users/456' \
///   -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiJ9...' \
///   -H 'Content-Type: application/json' \
///   -d '{"id": "789", "user_id": "456", "store_id": "123"}'
/// ```
#[put("/<store_id>/users/<user_id>", data = "<store_user>")]
pub async fn update_store_user(
    store_id: String,
    user_id: String,
    store_user: Json<StoreUser>,
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
    let owner_id = token.user_id;

    let store_user = store_user.into_inner();

    let check_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(owner_id)),
    ];
    let _ = match find_one_resource_where_fields!(Store, check_params).await {
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

    let query_params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("user_id", DatabaseValue::String(user_id.clone())),
    ];
    let _ = match find_one_resource_where_fields!(StoreUser, query_params).await {
        Ok(store_user) => store_user,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreUserError::StoreUserNotFound),
                    StoreUserError::StoreUserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    match update_resource!(
        StoreUser,
        store_user.id,
        vec![
            ("store_id", DatabaseValue::String(store_user.store_id)),
            ("user_id", DatabaseValue::String(store_user.user_id))
        ]
    )
    .await
    {
        Ok(store_user) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_user),
                Some("Store user updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreUserError::StoreUserUpdateFailed),
                StoreUserError::StoreUserUpdateFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Delete a store user
///
/// Removes a user from a store. Only the store owner can access this endpoint.
///
/// Required headers:
/// - Authorization: Bearer token
///
/// Path parameters:
/// - store_id: The ID of the store
/// - user_id: The ID of the user to delete
///
/// Returns:
/// - 200 OK: Store user deleted successfully
/// - 401 Unauthorized: Invalid or missing token
/// - 500 Internal Server Error: Failed to delete store user
///
/// Example:
/// ```bash
/// curl -X DELETE 'http://localhost:8000/api/stores/123/users/456' \
///   -H 'Authorization: Bearer eyJhbGciOiJIUzI1NiJ9...'
/// ```
#[delete("/<store_id>/users/<user_id>")]
pub async fn delete_store_user(
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

    let delete_params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("user_id", DatabaseValue::String(user_id.clone())),
    ];
    match delete_resource_where_fields!(StoreUser, delete_params).await {
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
                anyhow::anyhow!(StoreUserError::StoreUserDeletionFailed),
                StoreUserError::StoreUserDeletionFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}
