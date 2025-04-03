use crate::api::response::Response;
use crate::api::token::{RawToken, VerifiedToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::store_role::{StoreRole, StoreRoleError};
use crate::models::store_role_user::{StoreRoleUser, StoreRoleUserError};
use crate::{
    delete_resource_where_fields, find_one_resource_where_fields, insert_resource,
    join_all_resources_where_fields_on,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};

#[get("/<store_id>/users/<user_id>/roles")]
pub async fn get_store_user_roles(
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

    match join_all_resources_where_fields_on!(
        StoreRole,
        StoreUser,
        vec![
            ("store_id", DatabaseValue::String(store_id.clone())),
            ("user_id", DatabaseValue::String(user_id.clone()))
        ]
    )
    .await
    {
        Ok(roles) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(roles),
                Some("Store user roles fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleError::StoreRolesFetchFailed),
                StoreRoleError::StoreRolesFetchFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[post("/<store_id>/users/<user_id>/roles", data = "<store_role>")]
pub async fn create_store_user_role(
    store_id: String,
    user_id: String,
    store_role: Json<StoreRole>,
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

    let store_role_id = store_role.id.clone();
    let params = vec![("id", &store_role_id), ("store_id", &store_id)];
    let store_role = match find_one_resource_where_fields!(StoreRole, params).await {
        Ok(role) => role,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreRoleError::StoreRoleNotFound),
                    StoreRoleError::StoreRoleNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    };

    let insert_params = vec![
        ("store_id", DatabaseValue::String(store_id)),
        ("user_id", DatabaseValue::String(user_id)),
        ("role_id", DatabaseValue::String(store_role.id.clone())),
    ];
    match insert_resource!(StoreRoleUser, insert_params).await {
        Ok(_) => status::Custom(
            Status::Created,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_role),
                Some("Store user role created successfully".to_string()),
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

#[delete("/<store_id>/users/<user_id>/roles/<role_id>")]
pub async fn delete_store_user_role(
    store_id: String,
    user_id: String,
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

    match delete_resource_where_fields!(
        StoreRoleUser,
        vec![
            ("store_id", DatabaseValue::String(store_id.clone())),
            ("user_id", DatabaseValue::String(user_id.clone())),
            ("role_id", DatabaseValue::String(role_id.clone()))
        ]
    )
    .await
    {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(role_id.clone()),
                Some("Store user role deleted successfully".to_string()),
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
