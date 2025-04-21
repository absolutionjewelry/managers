-- Add up migration script here
ALTER TABLE store_galleries_images ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP;

CREATE INDEX idx_store_galleries_images_updated_at ON store_galleries_images (updated_at);