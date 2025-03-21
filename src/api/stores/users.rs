use crate::api::response::Response;
use crate::api::token::RawToken;
use crate::api::token::VerifiedToken;
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

#[get("/<store_id>/users")]
pub async fn get_store_users(store_id: String, token: RawToken) -> status::Custom<Value> {
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

    match find_all_resources_where_fields!(StoreUser, vec![("store_id", &store_id)]).await {
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

#[get("/<store_id>/users/<user_id>")]
pub async fn get_store_user(
    store_id: String,
    user_id: String,
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
    let owner_id = token.user_id;

    let _ = match find_one_resource_where_fields!(
        Store,
        vec![("id", &store_id), ("owner_id", &owner_id)]
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

    match find_one_resource_where_fields!(
        StoreUser,
        vec![("store_id", &store_id), ("user_id", &user_id)]
    )
    .await
    {
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

#[post("/<store_id>/users", data = "<store_user>")]
pub async fn create_store_user(
    store_id: String,
    store_user: Json<StoreUser>,
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

    let owner_id = token.user_id;

    let _ = match find_one_resource_where_fields!(
        Store,
        vec![("id", &store_id), ("owner_id", &owner_id)]
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

    let store_user = store_user.into_inner();

    match insert_resource!(
        StoreUser,
        vec![("store_id", &store_id), ("user_id", &store_user.user_id)]
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

#[put("/<store_id>/users/<user_id>", data = "<store_user>")]
pub async fn update_store_user(
    store_id: String,
    user_id: String,
    store_user: Json<StoreUser>,
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
    let owner_id = token.user_id;

    let store_user = store_user.into_inner();

    let _ = match find_one_resource_where_fields!(
        Store,
        vec![("id", &store_id), ("owner_id", &owner_id)]
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

    let _ = match find_one_resource_where_fields!(
        StoreUser,
        vec![("store_id", &store_id), ("user_id", &user_id)]
    )
    .await
    {
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
            ("store_id", &store_user.store_id),
            ("user_id", &store_user.user_id)
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

#[delete("/<store_id>/users/<user_id>")]
pub async fn delete_store_user(
    store_id: String,
    user_id: String,
    token: RawToken,
) -> status::Custom<Value> {
    let _ = match VerifiedToken::from_raw(token).await {
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
        StoreUser,
        vec![("store_id", &store_id), ("user_id", &user_id)]
    )
    .await
    {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(user_id),
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
