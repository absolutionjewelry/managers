use crate::api::response::Response;
use crate::api::token::{RawToken, VerifiedToken};
use crate::models::authentication::AuthenticationError;
use crate::models::store_role_user::{StoreRoleUser, StoreRoleUserError};
use crate::models::store_user::StoreUser;
use crate::{delete_resource_where_fields, insert_resource, join_all_resources_where_fields_on};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};

#[get("/<store_id>/roles/<role_id>/users")]
pub async fn get_store_role_users(
    store_id: String,
    role_id: String,
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

    match join_all_resources_where_fields_on!(
        StoreUser,
        StoreRole,
        vec![("store_id", &store_id), ("role_id", &role_id)]
    )
    .await
    {
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
                anyhow::anyhow!(StoreRoleUserError::StoreRoleUsersFetchFailed),
                StoreRoleUserError::StoreRoleUsersFetchFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[post("/<store_id>/roles/<role_id>/users", data = "<store_user>")]
pub async fn create_store_role_user(
    store_id: String,
    role_id: String,
    store_user: Json<StoreUser>,
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

    let store_user = store_user.into_inner();

    match insert_resource!(
        StoreRoleUser,
        vec![
            ("store_id", &store_id),
            ("role_id", &role_id),
            ("user_id", &store_user.id)
        ]
    )
    .await
    {
        Ok(store_user) => status::Custom(
            Status::Created,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_user),
                Some("Store user added to role successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleUserError::StoreRoleUserCreationFailed),
                StoreRoleUserError::StoreRoleUserCreationFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[delete("/<store_id>/roles/<role_id>/users/<user_id>")]
pub async fn delete_store_role_user(
    store_id: String,
    role_id: String,
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
        StoreRoleUser,
        vec![
            ("store_id", &store_id),
            ("role_id", &role_id),
            ("user_id", &user_id)
        ]
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
                anyhow::anyhow!(StoreRoleUserError::StoreRoleUserDeletionFailed),
                StoreRoleUserError::StoreRoleUserDeletionFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}
