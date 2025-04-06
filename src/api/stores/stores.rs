use crate::api::response::Response;
use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::store::{Store, StoreError};
use crate::{
    delete_resource_where_fields, find_all_archived_resources_where_fields,
    find_all_unarchived_resources_where_fields, find_one_resource_where_fields, insert_resource,
    update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use serde::{Deserialize, Serialize};

/// # Store Management API Endpoints
///
/// These endpoints handle CRUD operations for stores. All endpoints require authentication
/// via a bearer token in the Authorization header.
///
/// ## Authentication
/// All endpoints require a valid JWT token passed in the Authorization header:
/// ```text
/// Authorization: Bearer <your_jwt_token>
/// ```
///
/// ## Common Response Format
/// All endpoints return JSON responses in the following format:
/// ```json
/// {
///   "success": boolean,
///   "data": object | null,
///   "message": string | null,
///   "error": string | null
/// }
/// ```

/// Get all stores for the authenticated user
///
/// Returns a list of all stores (both archived and unarchived) owned by the authenticated user.
///
/// # Authorization
/// Requires a valid JWT token for a registered user
///
/// # Returns
/// - `200 OK` - List of stores successfully retrieved
/// - `401 Unauthorized` - Invalid or missing authentication token
/// - `404 Not Found` - No stores found for the user
///
/// # Example curl request:
/// ```bash
/// curl -X GET 'http://localhost:8000/api/stores' \
///   -H 'Authorization: Bearer your_token_here'
/// ```
///
/// # Example success response:
/// ```json
/// {
///   "success": true,
///   "data": [
///     {
///       "id": "store_id",
///       "storeName": "My Store",
///       "storeDescription": "Description",
///       "ownerId": "user_id",
///       "archived": false,
///       "createdAt": "2024-03-20T10:00:00Z",
///       "updatedAt": "2024-03-20T10:00:00Z"
///     }
///   ],
///   "message": "Stores fetched successfully"
/// }
/// ```
#[get("/")]
pub async fn get_stores(token: RawToken) -> status::Custom<Value> {
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
    let params = vec![("owner_id", DatabaseValue::String(user_id))];
    match find_all_unarchived_resources_where_fields!(Store, params).await {
        Ok(stores) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(stores),
                Some("Stores fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching stores: {:?}", err);
            status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreNotFound),
                    StoreError::StoreNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

/// Get only unarchived stores for the authenticated user
///
/// Returns a list of active (unarchived) stores owned by the authenticated user.
///
/// # Authorization
/// Requires a valid JWT token for a registered user
///
/// # Returns
/// - `200 OK` - List of unarchived stores successfully retrieved
/// - `401 Unauthorized` - Invalid or missing authentication token
/// - `404 Not Found` - No unarchived stores found
///
/// # Example curl request:
/// ```bash
/// curl -X GET 'http://localhost:8000/api/stores/unarchived' \
///   -H 'Authorization: Bearer your_token_here'
/// ```
#[get("/unarchived", rank = 2)]
pub async fn get_unarchived_stores(token: RawToken) -> status::Custom<Value> {
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
    let params = vec![("owner_id", DatabaseValue::String(user_id))];
    match find_all_unarchived_resources_where_fields!(Store, params).await {
        Ok(stores) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(stores),
                Some("Stores fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching unarchived stores: {:?}", err);
            status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreNotFound),
                    StoreError::StoreNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

/// Get archived stores for the authenticated user
///
/// Returns a list of archived stores owned by the authenticated user.
///
/// # Example curl request:
/// ```bash
/// curl -X GET 'http://localhost:8000/api/stores/archived' \
///   -H 'Authorization: Bearer your_token_here'
/// ```
#[get("/archived", rank = 1)]
pub async fn get_archived_stores(token: RawToken) -> status::Custom<Value> {
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
    let params = vec![("owner_id", DatabaseValue::String(user_id))];
    match find_all_archived_resources_where_fields!(Store, params).await {
        Ok(stores) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(stores),
                Some("Stores fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching archived stores: {:?}", err);
            status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreNotFound),
                    StoreError::StoreNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

/// Get a specific store by ID
///
/// Returns details for a single store if it belongs to the authenticated user.
///
/// # Example curl request:
/// ```bash
/// curl -X GET 'http://localhost:8000/api/stores/store_id_here' \
///   -H 'Authorization: Bearer your_token_here'
/// ```
#[get("/<store_id>", rank = 3)]
pub async fn get_store(store_id: String, token: RawToken) -> status::Custom<Value> {
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
    let params = vec![
        ("id", DatabaseValue::String(store_id)),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    match find_one_resource_where_fields!(Store, params).await {
        Ok(store) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store),
                Some("Store fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching store: {:?}", err);
            status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreNotFound),
                    StoreError::StoreNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

/// Data structure for creating a new store
///
/// # Fields
/// * `store_name` - Name of the store (required)
///   - Must not be empty
///   - Maximum length: 100 characters
/// * `store_description` - Optional description of the store
///   - Maximum length: 500 characters
///   - Can be null or omitted
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStore {
    pub store_name: String,
    pub store_description: Option<String>,
}

/// Create a new store
///
/// Creates a new store owned by the authenticated user.
///
/// # Authorization
/// Requires a valid JWT token for a registered user
///
/// # Request Body
/// ```json
/// {
///   "storeName": "My Store",
///   "storeDescription": "My awesome store description"  // optional
/// }
/// ```
///
/// # Returns
/// - `200 OK` - Store successfully created
/// - `401 Unauthorized` - Invalid or missing authentication token
/// - `500 Internal Server Error` - Failed to create store
///
/// # Example curl request:
/// ```bash
/// curl -X POST 'http://localhost:8000/api/stores' \
///   -H 'Authorization: Bearer your_token_here' \
///   -H 'Content-Type: application/json' \
///   -d '{
///     "storeName": "My Store",
///     "storeDescription": "My awesome store description"
///   }'
/// ```
///
/// # Example success response:
/// ```json
/// {
///   "success": true,
///   "data": {
///     "storeName": "My Store",
///     "storeDescription": "My awesome store description"
///   },
///   "message": "Store created successfully"
/// }
/// ```
#[post("/", data = "<store>")]
pub async fn create_store(store: Json<CreateStore>, token: RawToken) -> status::Custom<Value> {
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

    let params = vec![
        ("owner_id", DatabaseValue::String(user_id)),
        (
            "store_name",
            DatabaseValue::String(store.store_name.clone()),
        ),
        (
            "store_description",
            DatabaseValue::String(store.store_description.clone().unwrap_or_default()),
        ),
    ];
    match insert_resource!(Store, params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store.into_inner()),
                Some("Store created successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error creating store: {:?}", err);
            status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreCreationFailed),
                    StoreError::StoreCreationFailed.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

/// Data structure for updating an existing store
///
/// # Fields
/// * `store_name` - Updated name of the store (required)
///   - Must not be empty
///   - Maximum length: 100 characters
/// * `store_description` - Optional updated description of the store
///   - Maximum length: 500 characters
///   - Can be null to remove existing description
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStore {
    pub store_name: String,
    pub store_description: Option<String>,
}

/// Update an existing store
///
/// Updates the details of an existing store if it belongs to the authenticated user.
///
/// # Authorization
/// Requires a valid JWT token for a registered user
///
/// # URL Parameters
/// * `store_id` - The unique identifier of the store to update
///
/// # Request Body
/// ```json
/// {
///   "storeName": "Updated Store Name",
///   "storeDescription": "Updated store description"  // optional
/// }
/// ```
///
/// # Returns
/// - `200 OK` - Store successfully updated
/// - `401 Unauthorized` - Invalid or missing authentication token
/// - `404 Not Found` - Store not found or doesn't belong to user
/// - `500 Internal Server Error` - Failed to update store
///
/// # Example curl request:
/// ```bash
/// curl -X PUT 'http://localhost:8000/api/stores/store_id_here' \
///   -H 'Authorization: Bearer your_token_here' \
///   -H 'Content-Type: application/json' \
///   -d '{
///     "storeName": "Updated Store Name",
///     "storeDescription": "Updated store description"
///   }'
/// ```
#[put("/<store_id>", data = "<store>")]
pub async fn update_store(
    store_id: String,
    store: Json<UpdateStore>,
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

    let params = vec![
        (
            "store_name",
            DatabaseValue::String(store.store_name.clone()),
        ),
        (
            "store_description",
            DatabaseValue::String(store.store_description.clone().unwrap_or_default()),
        ),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    let store_id = store_id.clone();
    match update_resource!(Store, store_id, params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store.into_inner()),
                Some("Store updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error updating store: {:?}", err);
            status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreUpdateFailed),
                    StoreError::StoreUpdateFailed.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

/// Delete a store
///
/// Permanently deletes a store if it belongs to the authenticated user.
/// This action cannot be undone.
///
/// # Authorization
/// Requires a valid JWT token for a registered user
///
/// # URL Parameters
/// * `store_id` - The unique identifier of the store to delete
///
/// # Returns
/// - `200 OK` - Store successfully deleted
/// - `401 Unauthorized` - Invalid or missing authentication token
/// - `404 Not Found` - Store not found or doesn't belong to user
/// - `500 Internal Server Error` - Failed to delete store
///
/// # Example curl request:
/// ```bash
/// curl -X DELETE 'http://localhost:8000/api/stores/store_id_here' \
///   -H 'Authorization: Bearer your_token_here'
/// ```
///
/// # Example success response:
/// ```json
/// {
///   "success": true,
///   "data": "store_id_here",
///   "message": "Store deleted successfully"
/// }
/// ```
#[delete("/<store_id>")]
pub async fn delete_store(store_id: &str, token: RawToken) -> status::Custom<Value> {
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
    let user_id_str = user_id.as_str();
    let delete_params = vec![
        ("id", DatabaseValue::String(store_id.to_string())),
        ("owner_id", DatabaseValue::String(user_id_str.to_string())),
    ];
    let _ = match find_one_resource_where_fields!(Store, delete_params).await {
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

    match delete_resource_where_fields!(Store, delete_params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_id),
                Some("Store deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error deleting store: {:?}", err);
            status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreDeletionFailed),
                    StoreError::StoreDeletionFailed.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}
