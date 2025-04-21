-- Add up migration script here
ALTER TABLE store_galleries
ALTER COLUMN created_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE store_galleries
ALTER COLUMN updated_at TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE store_galleries
ALTER COLUMN archived_at TYPE TIMESTAMP WITH TIME ZONE;

