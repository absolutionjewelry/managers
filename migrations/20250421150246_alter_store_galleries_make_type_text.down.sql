-- Add down migration script here
ALTER TABLE store_galleries
ALTER COLUMN gallery_type TYPE VARCHAR(255);
