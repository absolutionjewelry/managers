use crate::api::response::Response;
use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::store_galleries_image::{StoreGalleriesImage, StoreGalleriesImageError};
use crate::models::store_gallery::{StoreGallery, StoreGalleryError};
use crate::{
    delete_resource_where_fields, find_all_unarchived_resources_where_fields,
    find_one_resource_where_fields, insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use serde::{Deserialize, Serialize};

#[get("/<store_id>/galleries/<gallery_id>/images")]
pub async fn get_images(
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

    let image_params = vec![
        ("id", DatabaseValue::String(gallery_id.to_string())),
        ("store_id", DatabaseValue::String(store_id.to_string())),
        (
            "store_gallery_id",
            DatabaseValue::String(gallery_id.to_string()),
        ),
    ];
    match find_all_unarchived_resources_where_fields!(StoreGalleriesImage, image_params).await {
        Ok(images) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(images),
                Some("Images fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching images: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleriesImageError::StoreGalleriesImageNotFound),
                    StoreGalleriesImageError::StoreGalleriesImageNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateImageRequest {
    pub image_id: String,
    pub image_position: i32,
}

#[post("/<store_id>/galleries/<gallery_id>/images", data = "<image>")]
pub async fn create_image(
    store_id: &str,
    gallery_id: &str,
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

    let gallery_params = vec![
        ("id", DatabaseValue::String(gallery_id.to_string())),
        ("store_id", DatabaseValue::String(store_id.to_string())),
    ];

    let gallery = find_one_resource_where_fields!(StoreGallery, gallery_params).await;

    if gallery.is_err() {
        return status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreGalleryError::StoreGalleryNotFound),
                StoreGalleryError::StoreGalleryNotFound.to_string(),
            ))
            .unwrap(),
        );
    }

    let gallery_image_params = vec![
        (
            "store_gallery_id",
            DatabaseValue::String(gallery_id.to_string()),
        ),
        ("store_id", DatabaseValue::String(store_id.to_string())),
        (
            "store_image_id",
            DatabaseValue::String(image.image_id.clone()),
        ),
        (
            "store_gallery_position",
            DatabaseValue::Int(image.image_position.to_string()),
        ),
        (
            "store_gallery_type",
            DatabaseValue::String(gallery.unwrap().gallery_type.unwrap().to_string()),
        ),
    ];

    match insert_resource!(StoreGalleriesImage, gallery_image_params).await {
        Ok(gallery_image) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(gallery_image),
                Some("Image created successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error creating image: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleriesImageError::StoreGalleriesImageCreationFailed),
                    StoreGalleriesImageError::StoreGalleriesImageCreationFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[delete("/<store_id>/galleries/<gallery_id>/images/<image_id>")]
pub async fn delete_image(
    store_id: &str,
    gallery_id: &str,
    image_id: &str,
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

    let gallery = find_one_resource_where_fields!(StoreGallery, gallery_params).await;

    if gallery.is_err() {
        return status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreGalleryError::StoreGalleryNotFound),
                StoreGalleryError::StoreGalleryNotFound.to_string(),
            ))
            .unwrap(),
        );
    }

    let gallery_image_params = vec![
        (
            "store_image_id",
            DatabaseValue::String(image_id.to_string()),
        ),
        ("store_id", DatabaseValue::String(store_id.to_string())),
        (
            "store_gallery_id",
            DatabaseValue::String(gallery_id.to_string()),
        ),
    ];

    match delete_resource_where_fields!(StoreGalleriesImage, gallery_image_params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!({}),
                Some("Image deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error deleting image: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleriesImageError::StoreGalleriesImageDeletionFailed),
                    StoreGalleriesImageError::StoreGalleriesImageDeletionFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateImageRequest {
    pub image_position: i32,
}

#[put(
    "/<store_id>/galleries/<gallery_id>/images/<image_id>",
    data = "<image>"
)]
pub async fn update_image(
    store_id: &str,
    gallery_id: &str,
    image_id: &str,
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

    let gallery_images_params = vec![
        (
            "store_image_id",
            DatabaseValue::String(image_id.to_string()),
        ),
        ("store_id", DatabaseValue::String(store_id.to_string())),
        (
            "store_gallery_id",
            DatabaseValue::String(gallery_id.to_string()),
        ),
    ];

    let gallery_image =
        find_one_resource_where_fields!(StoreGalleriesImage, gallery_images_params).await;

    if gallery_image.is_err() {
        return status::Custom(
            Status::NotFound,
            serde_json::to_value(&Response::error(
                anyhow::anyhow!(StoreGalleriesImageError::StoreGalleriesImageNotFound),
                StoreGalleriesImageError::StoreGalleriesImageNotFound.to_string(),
            ))
            .unwrap(),
        );
    }

    let gallery_image = gallery_image.unwrap();
    let gallery_image_params = vec![(
        "store_gallery_position",
        DatabaseValue::Int(image.image_position.to_string()),
    )];

    match update_resource!(StoreGalleriesImage, gallery_image.id, gallery_image_params).await {
        Ok(gallery_image) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(gallery_image),
                Some("Image updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error updating image: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleriesImageError::StoreGalleriesImageUpdateFailed),
                    StoreGalleriesImageError::StoreGalleriesImageUpdateFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[get("/<store_id>/galleries/<gallery_id>/images/<image_id>")]
pub async fn get_image(
    store_id: &str,
    gallery_id: &str,
    image_id: &str,
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

    let gallery_image_params = vec![
        (
            "store_image_id",
            DatabaseValue::String(image_id.to_string()),
        ),
        ("store_id", DatabaseValue::String(store_id.to_string())),
        (
            "store_gallery_id",
            DatabaseValue::String(gallery_id.to_string()),
        ),
    ];

    match find_one_resource_where_fields!(StoreGalleriesImage, gallery_image_params).await {
        Ok(gallery_image) => status::Custom(
            Status::Ok,
            serde_json::to_value(&Response::success(
                serde_json::json!(gallery_image),
                Some("Image fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching image: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(&Response::error(
                    anyhow::anyhow!(StoreGalleriesImageError::StoreGalleriesImageNotFound),
                    StoreGalleriesImageError::StoreGalleriesImageNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}
