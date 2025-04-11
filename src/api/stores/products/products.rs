use crate::api::response::Response;
use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::store::{Store, StoreError};
use crate::models::store_product::{StoreProduct, StoreProductError};
use crate::{
    delete_resource_where_fields, find_all_archived_resources_where_fields,
    find_all_resources_where_fields, find_all_unarchived_resources_where_fields,
    find_one_resource_where_fields, insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use serde::{Deserialize, Serialize};

/// Get all products for a store (both archived and unarchived)
///
/// Parameters:
/// - store_id: String (store identifier)
/// - token: RawToken (obtained from authentication)
///
/// Returns:
/// if success:
/// - status: 200
/// - data: Vec<Product>
/// else if unauthorized:
/// - status: 401
/// - error: AuthenticationError
/// else if store not found:
/// - status: 404
/// - error: StoreError
/// else if products not found:
/// - status: 404
/// - error: ProductError
///
/// Example:
/// ```
/// curl -X GET http://localhost:8000/api/stores/{store_id}/products \
///   -H 'Authorization: Bearer {token}'
/// ```

#[get("/<store_id>/products")]
pub async fn get_products(store_id: &str, token: RawToken) -> status::Custom<Value> {
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
        ("id", DatabaseValue::String(store_id.to_string())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
    ];
    let store_id = match find_one_resource_where_fields!(Store, store_params).await {
        Ok(store) => store.id,
        Err(err) => {
            println!("Error fetching store: {:?}", err);
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

    let product_params = vec![("store_id", DatabaseValue::String(store_id))];
    match find_all_unarchived_resources_where_fields!(StoreProduct, product_params).await {
        Ok(products) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(products),
                Some("Products fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching products: {:?}", err);
            status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreProductError::StoreProductNotFound),
                    StoreProductError::StoreProductNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

/// Get all unarchived products for a store
///
/// Parameters:
/// - store_id: String (store identifier)
/// - token: RawToken (obtained from authentication)
///
/// Returns:
/// if success:
/// - status: 200
/// - data: Vec<Product> (only unarchived products)
/// else if unauthorized:
/// - status: 401
/// - error: AuthenticationError
/// else if store not found:
/// - status: 404
/// - error: StoreError
/// else if products not found:
/// - status: 404
/// - error: ProductError
///
/// Example:
/// ```
/// curl -X GET http://localhost:8000/api/stores/{store_id}/products/unarchived \
///   -H 'Authorization: Bearer {token}'
/// ```
#[get("/<store_id>/products/unarchived", rank = 2)]
pub async fn get_unarchived_products(store_id: &str, token: RawToken) -> status::Custom<Value> {
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
        ("id", DatabaseValue::String(store_id.to_string())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
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

    let product_params = vec![("store_id", DatabaseValue::String(store_id))];
    match find_all_unarchived_resources_where_fields!(StoreProduct, product_params).await {
        Ok(products) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(products),
                Some("Products fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreProductError::StoreProductNotFound),
                StoreProductError::StoreProductNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Get all archived products for a store
///
/// Parameters:
/// - store_id: String (store identifier)
/// - token: RawToken (obtained from authentication)
///
/// Returns:
/// if success:
/// - status: 200
/// - data: Vec<Product> (only archived products)
/// else if unauthorized:
/// - status: 401
/// - error: AuthenticationError
/// else if store not found:
/// - status: 404
/// - error: StoreError
/// else if products not found:
/// - status: 404
/// - error: ProductError
///
/// Example:
/// ```
/// curl -X GET http://localhost:8000/api/stores/{store_id}/products/archived \
///   -H 'Authorization: Bearer {token}'
/// ```
#[get("/<store_id>/products/archived", rank = 1)]
pub async fn get_archived_products(store_id: &str, token: RawToken) -> status::Custom<Value> {
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
        ("id", DatabaseValue::String(store_id.to_string())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
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

    let product_params = vec![("store_id", DatabaseValue::String(store_id))];
    match find_all_archived_resources_where_fields!(StoreProduct, product_params).await {
        Ok(products) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(products),
                Some("Products fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreProductError::StoreProductNotFound),
                StoreProductError::StoreProductNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Get a specific product by ID
///
/// Parameters:
/// - store_id: String (store identifier)
/// - product_id: String (product identifier)
/// - token: RawToken (obtained from authentication)
///
/// Returns:
/// if success:
/// - status: 200
/// - data: Product
/// else if unauthorized:
/// - status: 401
/// - error: AuthenticationError
/// else if product not found:
/// - status: 404
/// - error: ProductError
///
/// Example:
/// ```
/// curl -X GET http://localhost:8000/api/stores/{store_id}/products/{product_id} \
///   -H 'Authorization: Bearer {token}'
/// ```
#[get("/<store_id>/products/<product_id>", rank = 3)]
pub async fn get_product(
    store_id: &str,
    product_id: &str,
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

    let product_params = vec![
        ("id", DatabaseValue::String(product_id.to_string())),
        ("store_id", DatabaseValue::String(store_id.to_string())),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    match find_one_resource_where_fields!(StoreProduct, product_params).await {
        Ok(product) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(product),
                Some("Product fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreProductError::StoreProductNotFound),
                StoreProductError::StoreProductNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductRequest {
    pub product_name: String,
    pub product_description: String,
    pub product_base_price: f64,
    pub product_base_cost: f64,
    pub product_base_quantity: f64,
}

/// Create a new product
///
/// Parameters:
/// - store_id: String (store identifier)
/// - product: Json<Product> (product data in request body)
/// - token: RawToken (obtained from authentication)
///
/// Request Body:
/// ```json
/// {
///   "product_name": "string",
///   "product_description": "string",
///   "product_base_price": float,
///   "product_base_quantity": integer
/// }
/// ```
///
/// Returns:
/// if success:
/// - status: 200
/// - data: Product (created product)
/// else if unauthorized:
/// - status: 401
/// - error: AuthenticationError
/// else if store not found:
/// - status: 404
/// - error: StoreError
/// else if creation fails:
/// - status: 500
/// - error: ProductError
///
/// Example:
/// ```
/// curl -X POST http://localhost:8000/api/stores/{store_id}/products \
///   -H 'Authorization: Bearer {token}' \
///   -H 'Content-Type: application/json' \
///   -d '{"product_name": "Test Product", "product_description": "Description", "product_base_price": 10.99, "product_base_quantity": 100}'
/// ```
#[post("/<store_id>/products", data = "<product>")]
pub async fn create_product(
    store_id: &str,
    product: Json<CreateProductRequest>,
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
        ("id", DatabaseValue::String(store_id.to_string())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
    ];
    let store_id = match find_one_resource_where_fields!(Store, store_params).await {
        Ok(store) => store.id,
        Err(err) => {
            println!("Error fetching store: {:?}", err);
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

    let product = product.into_inner();
    let product_params = vec![
        ("store_id", DatabaseValue::String(store_id)),
        (
            "product_name",
            DatabaseValue::String(product.product_name.clone()),
        ),
        (
            "product_description",
            DatabaseValue::String(product.product_description.clone()),
        ),
        (
            "product_base_price",
            DatabaseValue::Float(product.product_base_price.to_string()),
        ),
        (
            "product_base_cost",
            DatabaseValue::Float(product.product_base_cost.to_string()),
        ),
        (
            "product_base_quantity",
            DatabaseValue::Float(product.product_base_quantity.to_string()),
        ),
    ];

    match insert_resource!(StoreProduct, product_params).await {
        Ok(product) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(product),
                Some("Product created successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error creating product: {:?}", err);
            status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreProductError::StoreProductCreationFailed),
                    StoreProductError::StoreProductCreationFailed.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

/// Update an existing product
///
/// Parameters:
/// - store_id: String (store identifier)
/// - product_id: String (product identifier)
/// - product: Json<Product> (updated product data in request body)
/// - token: RawToken (obtained from authentication)
///
/// Request Body:
/// ```json
/// {
///   "product_name": "string",
///   "product_description": "string",
///   "product_base_price": float,
///   "product_base_cost": float,
///   "product_base_quantity": integer
/// }
/// ```
///
/// Returns:
/// if success:
/// - status: 200
/// - data: Product (updated product)
/// else if unauthorized:
/// - status: 401
/// - error: AuthenticationError
/// else if store not found:
/// - status: 404
/// - error: StoreError
/// else if update fails:
/// - status: 500
/// - error: ProductError
///
/// Example:
/// ```
/// curl -X PUT http://localhost:8000/api/stores/{store_id}/products/{product_id} \
///   -H 'Authorization: Bearer {token}' \
///   -H 'Content-Type: application/json' \
///   -d '{"product_name": "Updated Product", "product_description": "New Description", "product_base_price": 15.99, "product_base_cost": 8.99, "product_base_quantity": 50}'
/// ```
#[put("/<store_id>/products/<product_id>", data = "<product>")]
pub async fn update_product(
    store_id: &str,
    product_id: &str,
    product: Json<StoreProduct>,
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
        ("id", DatabaseValue::String(store_id.to_string())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
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

    let product = product.into_inner();
    let product_params = vec![
        (
            "product_name",
            DatabaseValue::String(product.product_name.unwrap()),
        ),
        (
            "product_description",
            DatabaseValue::String(product.product_description.unwrap()),
        ),
        (
            "product_base_price",
            DatabaseValue::Float(product.product_base_price.unwrap().to_string()),
        ),
        (
            "product_base_cost",
            DatabaseValue::Float(product.product_base_cost.unwrap().to_string()),
        ),
        (
            "product_base_quantity",
            DatabaseValue::Int(product.product_base_quantity.unwrap().to_string()),
        ),
    ];
    let product_id = product_id.to_string();
    match update_resource!(StoreProduct, product_id, product_params).await {
        Ok(product) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(product),
                Some("Product updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreProductError::StoreProductUpdateFailed),
                StoreProductError::StoreProductUpdateFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

/// Delete a product
///
/// Parameters:
/// - store_id: String (store identifier)
/// - product_id: String (product identifier)
/// - token: RawToken (obtained from authentication)
///
/// Returns:
/// if success:
/// - status: 200
/// - data: String (deleted product ID)
/// else if unauthorized:
/// - status: 401
/// - error: AuthenticationError
/// else if store not found:
/// - status: 404
/// - error: StoreError
/// else if deletion fails:
/// - status: 500
/// - error: ProductError
///
/// Example:
/// ```
/// curl -X DELETE http://localhost:8000/api/stores/{store_id}/products/{product_id} \
///   -H 'Authorization: Bearer {token}'
/// ```
#[delete("/<store_id>/products/<product_id>")]
pub async fn delete_product(
    store_id: &str,
    product_id: &str,
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
        ("id", DatabaseValue::String(store_id.to_string())),
        ("owner_id", DatabaseValue::String(user_id.clone())),
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

    let product_params = vec![
        ("id", DatabaseValue::String(product_id.to_string())),
        ("store_id", DatabaseValue::String(store_id)),
    ];
    match delete_resource_where_fields!(StoreProduct, product_params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(product_id),
                Some("Product deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreProductError::StoreProductDeletionFailed),
                StoreProductError::StoreProductDeletionFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}
