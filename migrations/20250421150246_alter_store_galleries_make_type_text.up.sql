-- Add up migration script here
ALTER TABLE store_galleries
ALTER COLUMN gallery_type TYPE TEXT;
