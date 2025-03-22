use crate::api::response::Response;
use crate::api::token::RawToken;
use crate::api::token::VerifiedToken;
use crate::models::authentication::AuthenticationError;
use crate::models::store::{Store, StoreError};
use crate::models::store_role::{StoreRole, StoreRoleError};
use crate::database::values::DatabaseValue;
use crate::{
    delete_resource_where_fields, find_all_resources_where_fields, find_one_resource_where_fields,
    insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};

#[get("/<store_id>/roles")]
pub async fn get_store_roles(store_id: String, token: RawToken) -> status::Custom<Value> {
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

    let params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone()))
    ];
    let _ = match find_one_resource_where_fields!(Store, params).await {
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

    match find_all_resources_where_fields!(StoreRole, params).await {
        Ok(store_roles) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_roles),
                Some("Store roles fetched successfully".to_string()),
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

#[get("/<store_id>/roles/<role_id>")]
pub async fn get_store_role(
    store_id: String,
    role_id: String,
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

    let params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone()))
    ];
    let _ = match find_one_resource_where_fields!(Store, params).await {
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

    let params = vec![("id", &role_id), ("store_id", &store_id)];
    match find_one_resource_where_fields!(StoreRole, params).await {
        Ok(store_role) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_role),
                Some("Store role fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleError::StoreRoleNotFound),
                StoreRoleError::StoreRoleNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[post("/<store_id>/roles", data = "<store_role>")]
pub async fn create_store_role(
    store_id: String,
    store_role: Json<StoreRole>,
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

    let params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone()))
    ];
    let _ = match find_one_resource_where_fields!(Store, params).await {
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

    let params = vec![
        ("store_id", DatabaseValue::String(store_role.store_id.clone())),
        ("role_name", DatabaseValue::String(store_role.role_name.clone())),
        ("role_description", DatabaseValue::String(store_role.role_description.clone())),
    ];
    match insert_resource!(StoreRole, params).await {
        Ok(store_role) => status::Custom(
            Status::Created,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_role),
                Some("Store role created successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleError::StoreRoleCreationFailed),
                StoreRoleError::StoreRoleCreationFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[put("/<store_id>/roles/<role_id>", data = "<store_role>")]
pub async fn update_store_role(
    store_id: String,
    role_id: String,
    store_role: Json<StoreRole>,
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

    let params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone()))
    ];
    let _ = match find_one_resource_where_fields!(Store, params).await {
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
        StoreRole,
        role_id,
        vec![
            ("role_name", &store_role.role_name),
            ("role_description", &store_role.role_description),
        ]
    )
    .await
    {
        Ok(store_role) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(store_role),
                Some("Store role updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleError::StoreRoleUpdateFailed),
                StoreRoleError::StoreRoleUpdateFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}

#[delete("/<store_id>/roles/<role_id>")]
pub async fn delete_store_role(
    store_id: String,
    role_id: String,
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

    let params = vec![
        ("store_id", DatabaseValue::String(store_id.clone())),
        ("owner_id", DatabaseValue::String(user_id.clone()))
    ];
    let _ = match find_one_resource_where_fields!(Store, params).await {
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

    let params = vec![("id", &role_id), ("store_id", &store_id)];
    let _ = match find_one_resource_where_fields!(StoreRole, params).await {
        Ok(store_role) => store_role,
        Err(_) => {
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreRoleError::StoreRoleNotFound),
                    StoreRoleError::StoreRoleNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    match delete_resource_where_fields!(StoreRole, vec![("id", &role_id), ("store_id", &store_id)])
        .await
    {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(role_id),
                Some("Store role deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(_) => status::Custom(
            Status::InternalServerError,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreRoleError::StoreRoleDeletionFailed),
                StoreRoleError::StoreRoleDeletionFailed.to_string(),
            ))
            .unwrap(),
        ),
    }
}
