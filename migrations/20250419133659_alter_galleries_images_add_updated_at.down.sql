-- Add down migration script here
DROP INDEX idx_store_galleries_images_updated_at;

ALTER TABLE store_galleries_images DROP COLUMN updated_at;