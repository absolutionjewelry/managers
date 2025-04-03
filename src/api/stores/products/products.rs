use crate::api::response::Response;
use crate::api::token::RawToken;
use crate::api::token::VerifiedToken;
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::product::Product;
use crate::models::product::ProductError;
use crate::models::store::{Store, StoreError};
use crate::{
    delete_resource_where_fields, find_all_archived_resources_where_fields,
    find_all_resources_where_fields, find_all_unarchived_resources_where_fields,
    find_one_resource_where_fields, insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};

#[get("/<store_id>/products")]
pub async fn get_products(store_id: String, token: RawToken) -> status::Custom<Value> {
    let token = match VerifiedToken::from_raw(token).await {
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
    match find_all_resources_where_fields!(Product, product_params).await {
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
                anyhow::anyhow!(ProductError::ProductNotFound),
                ProductError::ProductNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[get("/<store_id>/products/unarchived", rank = 2)]
pub async fn get_unarchived_products(store_id: String, token: RawToken) -> status::Custom<Value> {
    let token = match VerifiedToken::from_raw(token).await {
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
    match find_all_unarchived_resources_where_fields!(Product, product_params).await {
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
                anyhow::anyhow!(ProductError::ProductNotFound),
                ProductError::ProductNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[get("/<store_id>/products/archived", rank = 1)]
pub async fn get_archived_products(store_id: String, token: RawToken) -> status::Custom<Value> {
    let token = match VerifiedToken::from_raw(token).await {
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
    match find_all_archived_resources_where_fields!(Product, product_params).await {
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
                anyhow::anyhow!(ProductError::ProductNotFound),
                ProductError::ProductNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[get("/<store_id>/products/<product_id>", rank = 3)]
pub async fn get_product(
    store_id: String,
    product_id: String,
    token: RawToken,
) -> status::Custom<Value> {
    let token = match VerifiedToken::from_raw(token).await {
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
        ("id", DatabaseValue::String(product_id)),
        ("store_id", DatabaseValue::String(store_id)),
        ("owner_id", DatabaseValue::String(user_id)),
    ];
    match find_one_resource_where_fields!(Product, product_params).await {
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
                anyhow::anyhow!(ProductError::ProductNotFound),
                ProductError::ProductNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[post("/<store_id>/products", data = "<product>")]
pub async fn create_product(
    store_id: String,
    product: Json<Product>,
    token: RawToken,
) -> status::Custom<Value> {
    let token = match VerifiedToken::from_raw(token).await {
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

    let product = product.into_inner();
    let product_params = vec![
        ("store_id", DatabaseValue::String(store_id)),
        ("owner_id", DatabaseValue::String(user_id)),
        (
            "product_name",
            DatabaseValue::String(product.product_name.unwrap_or_default()),
        ),
        (
            "product_description",
            DatabaseValue::String(product.product_description.unwrap_or_default()),
        ),
        (
            "product_base_price",
            DatabaseValue::Float(product.product_base_price.unwrap_or_default().to_string()),
        ),
        (
            "product_base_quantity",
            DatabaseValue::Int(
                product
                    .product_base_quantity
                    .unwrap_or_default()
                    .to_string(),
            ),
        ),
    ];

    match insert_resource!(Product, product_params).await {
        Ok(product) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(product),
                Some("Product created successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(ProductError::ProductCreationFailed),
                ProductError::ProductCreationFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[put("/<store_id>/products/<product_id>", data = "<product>")]
pub async fn update_product(
    store_id: String,
    product_id: String,
    product: Json<Product>,
    token: RawToken,
) -> status::Custom<Value> {
    let token = match VerifiedToken::from_raw(token).await {
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
    let product_id = product_id.clone();
    match update_resource!(Product, product_id, product_params).await {
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
                anyhow::anyhow!(ProductError::ProductUpdateFailed),
                ProductError::ProductUpdateFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[delete("/<store_id>/products/<product_id>")]
pub async fn delete_product(
    store_id: String,
    product_id: String,
    token: RawToken,
) -> status::Custom<Value> {
    let token = match VerifiedToken::from_raw(token).await {
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
        ("id", DatabaseValue::String(product_id.clone())),
        ("store_id", DatabaseValue::String(store_id)),
    ];
    match delete_resource_where_fields!(Product, product_params).await {
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
                anyhow::anyhow!(ProductError::ProductDeletionFailed),
                ProductError::ProductDeletionFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}
