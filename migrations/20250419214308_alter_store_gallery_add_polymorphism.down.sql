-- Add down migration script here
DROP INDEX idx_store_galleries_gallery_type_id;
DROP INDEX idx_store_galleries_gallery_type;

ALTER TABLE store_galleries
DROP COLUMN gallery_type, DROP COLUMN gallery_type_id;
