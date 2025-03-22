use crate::api::response::Response;
use crate::api::token::RawToken;
use crate::api::token::VerifiedToken;
use crate::models::authentication::AuthenticationError;
use crate::models::store::{Store, StoreError};
use crate::{
    delete_resource_where_fields, find_all_archived_resources_where_fields,
    find_all_resources_where_fields, find_all_unarchived_resources_where_fields,
    find_one_resource_where_fields, insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};

#[get("/")]
pub async fn get_stores(token: RawToken) -> status::Custom<Value> {
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
            )
        }
    };

    let user_id = token.user_id;

    match find_all_resources_where_fields!(Store, vec![("owner_id", &user_id)]).await {
        Ok(stores) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(stores),
                Some("Stores fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreError::StoreNotFound),
                StoreError::StoreNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[get("/unarchived")]
pub async fn get_unarchived_stores(token: RawToken) -> status::Custom<Value> {
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
            )
        }
    };

    let user_id = token.user_id;

    match find_all_unarchived_resources_where_fields!(Store, vec![("owner_id", &user_id)]).await {
        Ok(stores) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(stores),
                Some("Stores fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreError::StoreNotFound),
                StoreError::StoreNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[get("/archived")]
pub async fn get_archived_stores(token: RawToken) -> status::Custom<Value> {
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
            )
        }
    };

    let user_id = token.user_id;

    match find_all_archived_resources_where_fields!(Store, vec![("owner_id", &user_id)]).await {
        Ok(stores) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(stores),
                Some("Stores fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreError::StoreNotFound),
                StoreError::StoreNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[get("/<store_id>", rank = 3)]
pub async fn get_store(store_id: String, token: RawToken) -> status::Custom<Value> {
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
            )
        }
    };

    let user_id = token.user_id;

    match find_one_resource_where_fields!(Store, vec![("id", &store_id), ("owner_id", &user_id)])
        .await
    {
        Ok(store) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store),
                Some("Store fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreError::StoreNotFound),
                StoreError::StoreNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[post("/", data = "<store>")]
pub async fn create_store(store: Json<Store>, token: RawToken) -> status::Custom<Value> {
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
            )
        }
    };
    let user_id = token.user_id;

    match insert_resource!(
        Store,
        vec![
            ("id", &store.id),
            ("owner_id", &user_id),
            ("store_name", &store.store_name),
            ("store_description", &store.store_description),
        ]
    )
    .await
    {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store.into_inner()),
                Some("Store created successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreError::StoreCreationFailed),
                StoreError::StoreCreationFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[put("/<store_id>", data = "<store>")]
pub async fn update_store(
    store_id: String,
    store: Json<Store>,
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
            )
        }
    };
    let user_id = token.user_id;

    let _ = match find_one_resource_where_fields!(
        Store,
        vec![("id", &store_id), ("owner_id", &user_id)]
    )
    .await
    {
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
        Store,
        &store_id,
        vec![
            ("store_name", &store.store_name),
            ("store_description", &store.store_description),
        ]
    )
    .await
    {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store.into_inner()),
                Some("Store updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreError::StoreUpdateFailed),
                StoreError::StoreUpdateFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[delete("/<store_id>")]
pub async fn delete_store(store_id: &str, token: RawToken) -> status::Custom<Value> {
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
            )
        }
    };
    let user_id = token.user_id;

    let _ = match find_one_resource_where_fields!(
        Store,
        vec![("id", &store_id), ("owner_id", &user_id.as_str())]
    )
    .await
    {
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

    match delete_resource_where_fields!(Store, vec![("id", &store_id)]).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_id),
                Some("Store deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreError::StoreDeletionFailed),
                StoreError::StoreDeletionFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}
