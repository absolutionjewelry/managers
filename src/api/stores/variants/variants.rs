//! Variants API endpoints
//!
//! This module provides REST API endpoints for managing product variants within a store.
//! Variants represent different versions or configurations of products (e.g. sizes, colors, materials).
//!
//! # Authentication
//! All endpoints in this module require authentication via a bearer token. The token must be valid
//! and belong to the owner of the store being accessed.
//!
//! # Common Error Responses
//! - 401 Unauthorized: Invalid or missing authentication token
//! - 404 Not Found: Store or variant not found
//! - 500 Internal Server Error: Database or server-side errors
//!
//! # Resource Structure
//! Variants have the following main attributes:
//! - variant_name: Name/identifier of the variant (e.g. "Large", "Red")
//! - variant_description: Detailed description of the variant
//! - variant_base_cost: Base cost for the variant
//! - variant_base_price: Retail price for the variant
//! - variant_base_quantity: Available stock quantity

use crate::api::response::Response;
use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::{
    authentication::AuthenticationError,
    store::{Store, StoreError},
    variant::{Variant, VariantError},
};
use crate::{
    delete_resource_where_fields, find_all_archived_resources_where_fields,
    find_all_resources_where_fields, find_all_unarchived_resources_where_fields,
    find_one_resource_where_fields, insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};

/// Get all variants for a store
///
/// Retrieves a list of all variants (both archived and unarchived) belonging to the specified store.
/// Results are returned in chronological order by creation date.
///
/// # Authorization
/// Requires a valid authentication token belonging to the store owner.
///
/// # Path Parameters
/// * `store_id` - Unique identifier of the store
///
/// # Query Parameters
/// None
///
/// # Response Format
/// Returns a JSON array of variant objects on success:
/// ```json
/// {
///   "data": [
///     {
///       "id": "variant_123",
///       "variant_name": "Large",
///       "variant_description": "Large size option",
///       "variant_base_cost": 10.00,
///       "variant_base_price": 19.99,
///       "variant_base_quantity": 100,
///       "archived": false,
///       "created_at": "2024-01-01T00:00:00Z",
///       "updated_at": "2024-01-01T00:00:00Z"
///     },
///     // ... additional variants
///   ],
///   "message": "Variants fetched successfully"
/// }
/// ```
///
/// # Errors
/// * 401 Unauthorized - Invalid authentication token
/// * 404 Not Found - Store not found or no variants exist
///
/// # Example
/// ```bash
/// curl -X GET \
///   'http://localhost:8000/api/stores/store_123/variants' \
///   -H 'Authorization: Bearer your_token_here'
/// ```
#[get("/<store_id>/variants")]
pub async fn get_variants(store_id: String, token: RawToken) -> status::Custom<Value> {
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
            );
        }
    };

    let user_id = token.user_id;

    let store_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    let store_id = match find_one_resource_where_fields!(Store, store_params).await {
        Ok(store) => store.id,
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

    let variant_params = vec![("store_id", DatabaseValue::String(store_id))];
    let variants = match find_all_resources_where_fields!(Variant, variant_params).await {
        Ok(variants) => variants,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(VariantError::VariantNotFound),
                    VariantError::VariantNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    status::Custom(
        Status::Ok,
        serde_json::to_value(&Response::success(
            serde_json::json!(variants),
            Some("Variants fetched successfully".to_string()),
        ))
        .unwrap(),
    )
}

/// Get archived variants for a store
///
/// Returns a list of only archived variants belonging to the specified store.
/// Requires authentication and store ownership verification.
///
/// # Arguments
/// * `store_id` - The ID of the store to get archived variants from
/// * `token` - Authentication token
///
/// # Returns
/// * Success (200) - List of archived variants with success message
/// * Unauthorized (401) - If authentication token is invalid
/// * Not Found (404) - If store not found or no archived variants exist
///
/// # Example
/// ```bash
/// curl -X GET \
///   'http://localhost:8000/api/stores/store_123/variants/archived' \
///   -H 'Authorization: Bearer your_token_here'
/// ```
#[get("/<store_id>/variants/archived", rank = 1)]
pub async fn get_archived_variants(store_id: String, token: RawToken) -> status::Custom<Value> {
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
            );
        }
    };

    let user_id = token.user_id;

    let store_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    let store_id = match find_one_resource_where_fields!(Store, store_params).await {
        Ok(store) => store.id,
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

    let variant_params = vec![("store_id", DatabaseValue::String(store_id))];
    let variants = match find_all_archived_resources_where_fields!(Variant, variant_params).await {
        Ok(variants) => variants,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(VariantError::VariantNotFound),
                    VariantError::VariantNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    status::Custom(
        Status::Ok,
        serde_json::to_value(&Response::success(
            serde_json::json!(variants),
            Some("Archived variants fetched successfully".to_string()),
        ))
        .unwrap(),
    )
}

/// Get unarchived variants for a store
///
/// Returns a list of only unarchived variants belonging to the specified store.
/// Requires authentication and store ownership verification.
///
/// # Arguments
/// * `store_id` - The ID of the store to get unarchived variants from
/// * `token` - Authentication token
///
/// # Returns
/// * Success (200) - List of unarchived variants with success message
/// * Unauthorized (401) - If authentication token is invalid
/// * Not Found (404) - If store not found or no unarchived variants exist
///
/// # Example
/// ```bash
/// curl -X GET \
///   'http://localhost:8000/api/stores/store_123/variants/unarchived' \
///   -H 'Authorization: Bearer your_token_here'
/// ```
#[get("/<store_id>/variants/unarchived", rank = 2)]
pub async fn get_unarchived_variants(store_id: String, token: RawToken) -> status::Custom<Value> {
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
            );
        }
    };

    let user_id = token.user_id;

    let store_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    let store_id = match find_one_resource_where_fields!(Store, store_params).await {
        Ok(store) => store.id,
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

    let variant_params = vec![("store_id", DatabaseValue::String(store_id))];
    let variants = match find_all_unarchived_resources_where_fields!(Variant, variant_params).await
    {
        Ok(variants) => variants,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(VariantError::VariantNotFound),
                    VariantError::VariantNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    status::Custom(
        Status::Ok,
        serde_json::to_value(&Response::success(
            serde_json::json!(variants),
            Some("Unarchived variants fetched successfully".to_string()),
        ))
        .unwrap(),
    )
}

/// Get a specific variant by ID
///
/// Returns details for a single variant identified by its ID.
/// Requires authentication and store ownership verification.
///
/// # Arguments
/// * `store_id` - The ID of the store containing the variant
/// * `variant_id` - The ID of the variant to retrieve
/// * `token` - Authentication token
///
/// # Returns
/// * Success (200) - Variant details with success message
/// * Unauthorized (401) - If authentication token is invalid
/// * Not Found (404) - If store or variant not found
///
/// # Example
/// ```bash
/// curl -X GET \
///   'http://localhost:8000/api/stores/store_123/variants/variant_456' \
///   -H 'Authorization: Bearer your_token_here'
/// ```
#[get("/<store_id>/variants/<variant_id>", rank = 3)]
pub async fn get_variant(
    store_id: String,
    variant_id: String,
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
            );
        }
    };

    let user_id = token.user_id;

    let store_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    let store_id = match find_one_resource_where_fields!(Store, store_params).await {
        Ok(store) => store.id,
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

    let variant_params = vec![
        ("id", DatabaseValue::String(variant_id)),
        ("store_id", DatabaseValue::String(store_id)),
    ];
    let variant = match find_one_resource_where_fields!(Variant, variant_params).await {
        Ok(variant) => variant,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(VariantError::VariantNotFound),
                    VariantError::VariantNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    status::Custom(
        Status::Ok,
        serde_json::to_value(&Response::success(
            serde_json::json!(variant),
            Some("Variant fetched successfully".to_string()),
        ))
        .unwrap(),
    )
}

/// Create a new variant
///
/// Creates a new variant in the specified store with the provided details.
/// Requires authentication and store ownership verification.
///
/// # Arguments
/// * `store_id` - The ID of the store to create the variant in
/// * `variant` - JSON payload containing variant details
/// * `token` - Authentication token
///
/// # Returns
/// * Success (200) - Created variant details with success message
/// * Unauthorized (401) - If authentication token is invalid
/// * Not Found (404) - If store not found
/// * Internal Server Error (500) - If variant creation fails
///
/// # Example
/// ```bash
/// curl -X POST \
///   'http://localhost:8000/api/stores/store_123/variants' \
///   -H 'Authorization: Bearer your_token_here' \
///   -H 'Content-Type: application/json' \
///   -d '{
///     "variant_name": "Large",
///     "variant_description": "Large size option",
///     "variant_base_cost": 10.00,
///     "variant_base_price": 19.99,
///     "variant_base_quantity": 100
///   }'
/// ```
#[post("/<store_id>/variants", data = "<variant>")]
pub async fn create_variant(
    store_id: String,
    variant: Json<Variant>,
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
            );
        }
    };

    let user_id = token.user_id;

    let store_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    let _ = match find_one_resource_where_fields!(Store, store_params).await {
        Ok(store) => store.id,
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

    let variant = variant.into_inner();
    let variant_params = vec![
        (
            "variant_name",
            DatabaseValue::String(variant.variant_name.unwrap()),
        ),
        (
            "variant_description",
            DatabaseValue::String(variant.variant_description.unwrap()),
        ),
        (
            "variant_base_cost",
            DatabaseValue::Float(variant.variant_base_cost.unwrap().to_string()),
        ),
        (
            "variant_base_price",
            DatabaseValue::Float(variant.variant_base_price.unwrap().to_string()),
        ),
        (
            "variant_base_quantity",
            DatabaseValue::Int(variant.variant_base_quantity.unwrap().to_string()),
        ),
    ];

    let variant = match insert_resource!(Variant, variant_params).await {
        Ok(variant) => variant,
        Err(_) => {
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(VariantError::VariantCreationFailed),
                    VariantError::VariantCreationFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    status::Custom(
        Status::Ok,
        serde_json::to_value(&Response::success(
            serde_json::json!(variant),
            Some("Variant created successfully".to_string()),
        ))
        .unwrap(),
    )
}

/// Update an existing variant
///
/// Updates the details of an existing variant identified by its ID.
/// Requires authentication and store ownership verification.
///
/// # Arguments
/// * `store_id` - The ID of the store containing the variant
/// * `variant_id` - The ID of the variant to update
/// * `variant` - JSON payload containing updated variant details
/// * `token` - Authentication token
///
/// # Returns
/// * Success (200) - Updated variant details with success message
/// * Unauthorized (401) - If authentication token is invalid
/// * Not Found (404) - If store not found
/// * Internal Server Error (500) - If variant update fails
///
/// # Example
/// ```bash
/// curl -X PUT \
///   'http://localhost:8000/api/stores/store_123/variants/variant_456' \
///   -H 'Authorization: Bearer your_token_here' \
///   -H 'Content-Type: application/json' \
///   -d '{
///     "variant_name": "Extra Large",
///     "variant_description": "Extra large size option",
///     "variant_base_cost": 12.00,
///     "variant_base_price": 24.99,
///     "variant_base_quantity": 50
///   }'
/// ```
#[put("/<store_id>/variants/<variant_id>", data = "<variant>")]
pub async fn update_variant(
    store_id: String,
    variant_id: String,
    variant: Json<Variant>,
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
            );
        }
    };

    let user_id = token.user_id;

    let store_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    let _ = match find_one_resource_where_fields!(Store, store_params).await {
        Ok(store) => store.id,
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

    let variant = variant.into_inner();
    let variant_params = vec![
        (
            "variant_name",
            DatabaseValue::String(variant.variant_name.unwrap()),
        ),
        (
            "variant_description",
            DatabaseValue::String(variant.variant_description.unwrap()),
        ),
        (
            "variant_base_cost",
            DatabaseValue::Float(variant.variant_base_cost.unwrap().to_string()),
        ),
        (
            "variant_base_price",
            DatabaseValue::Float(variant.variant_base_price.unwrap().to_string()),
        ),
        (
            "variant_base_quantity",
            DatabaseValue::Int(variant.variant_base_quantity.unwrap().to_string()),
        ),
    ];

    let variant_id = variant_id.clone();
    let variant_params = variant_params.clone();
    let variant = match update_resource!(Variant, variant_id, variant_params).await {
        Ok(variant) => variant,
        Err(_) => {
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(VariantError::VariantUpdateFailed),
                    VariantError::VariantUpdateFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    status::Custom(
        Status::Ok,
        serde_json::to_value(&Response::success(
            serde_json::json!(variant),
            Some("Variant updated successfully".to_string()),
        ))
        .unwrap(),
    )
}

/// Delete a variant
///
/// Deletes a variant identified by its ID from the specified store.
/// Requires authentication and store ownership verification.
///
/// # Arguments
/// * `store_id` - The ID of the store containing the variant
/// * `variant_id` - The ID of the variant to delete
/// * `token` - Authentication token
///
/// # Returns
/// * Success (200) - Success message
/// * Unauthorized (401) - If authentication token is invalid
/// * Not Found (404) - If store not found
/// * Internal Server Error (500) - If variant deletion fails
///
/// # Example
/// ```bash
/// curl -X DELETE \
///   'http://localhost:8000/api/stores/store_123/variants/variant_456' \
///   -H 'Authorization: Bearer your_token_here'
/// ```
#[delete("/<store_id>/variants/<variant_id>")]
pub async fn delete_variant(
    store_id: String,
    variant_id: String,
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
            );
        }
    };

    let user_id = token.user_id;

    let store_params = vec![
        ("id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    let _ = match find_one_resource_where_fields!(Store, store_params).await {
        Ok(store) => store.id,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreError::StoreNotFound),
                    StoreError::StoreNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    };

    let variant_params = vec![
        ("id", DatabaseValue::String(variant_id.clone())),
        ("store_id", DatabaseValue::String(store_id.clone())),
    ];
    match delete_resource_where_fields!(Variant, variant_params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(()),
                Some("Variant deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(VariantError::VariantDeletionFailed),
                VariantError::VariantDeletionFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}
