-- Add down migration script here
ALTER TABLE store_products
ALTER COLUMN created_at TYPE TIMESTAMP;

ALTER TABLE store_products
ALTER COLUMN updated_at TYPE TIMESTAMP;

ALTER TABLE store_products
ALTER COLUMN archived_at TYPE TIMESTAMP;
