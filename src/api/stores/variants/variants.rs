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
