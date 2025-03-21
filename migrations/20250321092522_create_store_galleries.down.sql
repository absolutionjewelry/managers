-- Add down migration script here
DROP INDEX IF EXISTS idx_store_galleries_images_store_image_id;
DROP INDEX IF EXISTS idx_store_galleries_images_store_gallery_type;
DROP INDEX IF EXISTS idx_store_galleries_images_store_gallery_position;
DROP INDEX IF EXISTS idx_store_galleries_images_created_at;
DROP INDEX IF EXISTS idx_store_galleries_images_store_gallery_id;
DROP INDEX IF EXISTS idx_store_galleries_images_store_id;
COMMENT ON COLUMN store_galleries_images.store_gallery_type IS NULL;
DROP TABLE IF EXISTS store_galleries_images;

DROP INDEX IF EXISTS idx_store_galleries_archived_at;
DROP INDEX IF EXISTS idx_store_galleries_updated_at;
DROP INDEX IF EXISTS idx_store_galleries_created_at;
DROP INDEX IF EXISTS idx_store_galleries_gallery_name;
DROP INDEX IF EXISTS idx_store_galleries_store_id;
DROP TABLE IF EXISTS store_galleries;

DROP INDEX IF EXISTS idx_store_images_image_name;
DROP INDEX IF EXISTS idx_store_images_image_content_type;
DROP INDEX IF EXISTS idx_store_images_image_content_length;
DROP INDEX IF EXISTS idx_store_images_created_at;
DROP INDEX IF EXISTS idx_store_images_updated_at;
DROP INDEX IF EXISTS idx_store_images_archived_at;
DROP INDEX IF EXISTS idx_store_images_store_id;
DROP TABLE IF EXISTS store_images;
