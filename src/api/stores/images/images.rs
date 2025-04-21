use crate::api::response::Response;
use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::store_image::{StoreImage, StoreImageError};
use crate::{
    delete_resource_where_fields, find_all_archived_resources_where_fields,
    find_all_unarchived_resources_where_fields, find_one_resource_where_fields, insert_resource,
    update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use serde::{Deserialize, Serialize};

#[get("/<store_id>/images")]
pub async fn get_images(store_id: &str, token: RawToken) -> status::Custom<Value> {
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

    match find_all_unarchived_resources_where_fields!(StoreImage, store_params).await {
        Ok(images) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(images),
                Some("Images fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error fetching images: {:?}", e);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreImageError::StoreImageNotFound),
                    StoreImageError::StoreImageNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[get("/<store_id>/images/archived")]
pub async fn get_archived_images(store_id: &str, token: RawToken) -> status::Custom<Value> {
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

    match find_all_archived_resources_where_fields!(StoreImage, store_params).await {
        Ok(images) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(images),
                Some("Archived images fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error fetching archived images: {:?}", e);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreImageError::StoreImageNotFound),
                    StoreImageError::StoreImageNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[get("/<store_id>/images/unarchived")]
pub async fn get_unarchived_images(store_id: &str, token: RawToken) -> status::Custom<Value> {
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

    match find_all_unarchived_resources_where_fields!(StoreImage, store_params).await {
        Ok(images) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(images),
                Some("Unarchived images fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error fetching unarchived images: {:?}", e);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreImageError::StoreImageNotFound),
                    StoreImageError::StoreImageNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[get("/<store_id>/images/<id>", rank = 3)]
pub async fn get_image(store_id: &str, id: &str, token: RawToken) -> status::Custom<Value> {
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

    let image_params = vec![
        ("id", DatabaseValue::String(id.to_string())),
        ("store_id", DatabaseValue::String(store_id.to_string())),
    ];

    match find_one_resource_where_fields!(StoreImage, image_params).await {
        Ok(image) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(image),
                Some("Image fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error fetching image: {:?}", e);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreImageError::StoreImageNotFound),
                    StoreImageError::StoreImageNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateImageRequest {
    pub image_name: String,
    pub image_description: String,
    pub image_content_type: String,
    pub image_content: String,
}

#[post("/<store_id>/images", data = "<image>")]
pub async fn create_image(
    store_id: &str,
    image: Json<CreateImageRequest>,
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

    let image_params = vec![
        ("store_id", DatabaseValue::String(store_id.to_string())),
        (
            "image_name",
            DatabaseValue::String(image.image_name.clone()),
        ),
        (
            "image_description",
            DatabaseValue::String(image.image_description.clone()),
        ),
        (
            "image_content_type",
            DatabaseValue::String(image.image_content_type.clone()),
        ),
        (
            "image_content",
            DatabaseValue::String(image.image_content.clone()),
        ),
    ];

    match insert_resource!(StoreImage, image_params).await {
        Ok(image) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(image),
                Some("Image created successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error creating image: {:?}", e);
            return status::Custom(
                Status::BadRequest,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreImageError::StoreImageCreationFailed),
                    StoreImageError::StoreImageCreationFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateImageRequest {
    pub image_name: Option<String>,
    pub image_description: Option<String>,
    pub image_content_type: Option<String>,
    pub image_content: Option<String>,
}

#[put("/<store_id>/images/<id>", data = "<image>")]
pub async fn update_image(
    store_id: &str,
    id: &str,
    image: Json<UpdateImageRequest>,
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

    let image_params = vec![
        ("id", DatabaseValue::String(id.to_string())),
        ("store_id", DatabaseValue::String(store_id.to_string())),
        (
            "image_name",
            DatabaseValue::String(image.image_name.clone().unwrap()),
        ),
        (
            "image_description",
            DatabaseValue::String(image.image_description.clone().unwrap()),
        ),
        (
            "image_content_type",
            DatabaseValue::String(image.image_content_type.clone().unwrap()),
        ),
        (
            "image_content",
            DatabaseValue::String(image.image_content.clone().unwrap()),
        ),
    ];

    match update_resource!(StoreImage, id, image_params).await {
        Ok(image) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(image),
                Some("Image updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error updating image: {:?}", e);
            return status::Custom(
                Status::BadRequest,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreImageError::StoreImageUpdateFailed),
                    StoreImageError::StoreImageUpdateFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[delete("/<store_id>/images/<id>")]
pub async fn delete_image(store_id: &str, id: &str, token: RawToken) -> status::Custom<Value> {
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

    let image_params = vec![
        ("id", DatabaseValue::String(id.to_string())),
        ("store_id", DatabaseValue::String(store_id.to_string())),
    ];

    match delete_resource_where_fields!(StoreImage, image_params).await {
        Ok(image) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(image),
                Some("Image deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(e) => {
            println!("Error deleting image: {:?}", e);
            return status::Custom(
                Status::BadRequest,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreImageError::StoreImageDeletionFailed),
                    StoreImageError::StoreImageDeletionFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}
