use crate::api::response::Response;
use crate::api::token::RawToken;
use crate::api::token::VerifiedToken;
use crate::models::authentication::AuthenticationError;
use crate::models::product::Product;
use crate::models::product::ProductError;
use crate::models::store::{Store, StoreError};
use crate::{
    find_all_archived_resources_where_fields, find_all_resources_where_fields,
    find_all_unarchived_resources_where_fields, find_one_resource_where_fields, insert_resource,
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

    let store_id = match find_one_resource_where_fields!(
        Store,
        vec![("id", &store_id), ("owner_id", &user_id)]
    )
    .await
    {
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

    match find_all_resources_where_fields!(Product, vec![("store_id", &store_id)]).await {
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

#[get("/<store_id>/products/unarchived")]
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

    let store_id = match find_one_resource_where_fields!(
        Store,
        vec![("id", &store_id), ("owner_id", &user_id)]
    )
    .await
    {
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

    match find_all_unarchived_resources_where_fields!(Product, vec![("store_id", &store_id)]).await
    {
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

#[get("/<store_id>/products/archived")]
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

    let store_id = match find_one_resource_where_fields!(
        Store,
        vec![("id", &store_id), ("owner_id", &user_id)]
    )
    .await
    {
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

    match find_all_archived_resources_where_fields!(Product, vec![("store_id", &store_id)]).await {
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

    match find_one_resource_where_fields!(
        Product,
        vec![
            ("id", &product_id),
            ("store_id", &store_id),
            ("owner_id", &user_id)
        ]
    )
    .await
    {
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
