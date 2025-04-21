use crate::api::response::Response;
use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::store::{Store, StoreError};
use crate::models::store_gallery::{StoreGallery, StoreGalleryError};
use crate::{
    delete_resource_where_fields, find_all_archived_resources_where_fields,
    find_all_resources_where_fields, find_all_unarchived_resources_where_fields,
    find_one_resource_where_fields, insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use serde::{Deserialize, Serialize};

#[get("/<store_id>/galleries")]
pub async fn get_galleries(store_id: &str, token: RawToken) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
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

    let store_params = vec![("id", DatabaseValue::String(store_id.to_string()))];

    let gallery_params = vec![("store_id", DatabaseValue::String(store_id.to_string()))];

    match find_all_unarchived_resources_where_fields!(StoreGallery, gallery_params).await {
        Ok(galleries) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(galleries),
                Some("Galleries fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error fetching galleries: {:?}", e);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleryError::StoreGalleryNotFound),
                    StoreGalleryError::StoreGalleryNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[get("/<store_id>/galleries/archived")]
pub async fn get_archived_galleries(store_id: &str, token: RawToken) -> status::Custom<Value> {
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

    let gallery_params = vec![("store_id", DatabaseValue::String(store_id.to_string()))];

    match find_all_archived_resources_where_fields!(StoreGallery, gallery_params).await {
        Ok(galleries) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(galleries),
                Some("Archived galleries fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error fetching archived galleries: {:?}", e);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleryError::StoreGalleryNotFound),
                    StoreGalleryError::StoreGalleryNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[get("/<store_id>/galleries/unarchived")]
pub async fn get_unarchived_galleries(store_id: &str, token: RawToken) -> status::Custom<Value> {
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

    let gallery_params = vec![("store_id", DatabaseValue::String(store_id.to_string()))];

    match find_all_unarchived_resources_where_fields!(StoreGallery, gallery_params).await {
        Ok(galleries) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(galleries),
                Some("Unarchived galleries fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error fetching unarchived galleries: {:?}", e);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleryError::StoreGalleryNotFound),
                    StoreGalleryError::StoreGalleryNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[get("/<store_id>/galleries/<id>", rank = 3)]
pub async fn get_gallery(store_id: &str, id: &str, token: RawToken) -> status::Custom<Value> {
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

    let gallery_params = vec![
        ("id", DatabaseValue::String(id.to_string())),
        ("store_id", DatabaseValue::String(store_id.to_string())),
    ];

    match find_one_resource_where_fields!(StoreGallery, gallery_params).await {
        Ok(gallery) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(gallery),
                Some("Gallery fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error fetching gallery: {:?}", e);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleryError::StoreGalleryNotFound),
                    StoreGalleryError::StoreGalleryNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGalleryRequest {
    pub gallery_name: String,
    pub gallery_description: String,
}

#[post("/<store_id>/galleries", data = "<gallery>")]
pub async fn create_gallery(
    store_id: &str,
    gallery: Json<CreateGalleryRequest>,
    token: RawToken,
) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
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

    let gallery_params = vec![
        ("store_id", DatabaseValue::String(store_id.to_string())),
        (
            "gallery_name",
            DatabaseValue::String(gallery.gallery_name.clone()),
        ),
        (
            "gallery_description",
            DatabaseValue::String(gallery.gallery_description.clone()),
        ),
    ];

    match insert_resource!(StoreGallery, gallery_params).await {
        Ok(gallery) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(gallery),
                Some("Gallery created successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error creating gallery: {:?}", e);
            return status::Custom(
                Status::BadRequest,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleryError::StoreGalleryCreationFailed),
                    StoreGalleryError::StoreGalleryCreationFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGalleryRequest {
    pub gallery_name: Option<String>,
    pub gallery_description: Option<String>,
}

#[put("/<store_id>/galleries/<gallery_id>", data = "<gallery>")]
pub async fn update_gallery(
    store_id: &str,
    gallery_id: &str,
    gallery: Json<UpdateGalleryRequest>,
    token: RawToken,
) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
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

    let gallery_params = vec![
        ("id", DatabaseValue::String(gallery_id.to_string())),
        ("store_id", DatabaseValue::String(store_id.to_string())),
        (
            "gallery_name",
            DatabaseValue::String(gallery.gallery_name.clone().unwrap()),
        ),
        (
            "gallery_description",
            DatabaseValue::String(gallery.gallery_description.clone().unwrap()),
        ),
    ];

    match update_resource!(StoreGallery, gallery_id, gallery_params).await {
        Ok(gallery) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(gallery),
                Some("Gallery updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error updating gallery: {:?}", e);
            return status::Custom(
                Status::BadRequest,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleryError::StoreGalleryUpdateFailed),
                    StoreGalleryError::StoreGalleryUpdateFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[delete("/<store_id>/galleries/<gallery_id>")]
pub async fn delete_gallery(
    store_id: &str,
    gallery_id: &str,
    token: RawToken,
) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
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

    let gallery_params = vec![
        ("id", DatabaseValue::String(gallery_id.to_string())),
        ("store_id", DatabaseValue::String(store_id.to_string())),
    ];

    match delete_resource_where_fields!(StoreGallery, gallery_params).await {
        Ok(gallery) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(gallery),
                Some("Gallery deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error deleting gallery: {:?}", e);
            return status::Custom(
                Status::BadRequest,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleryError::StoreGalleryDeletionFailed),
                    StoreGalleryError::StoreGalleryDeletionFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}
