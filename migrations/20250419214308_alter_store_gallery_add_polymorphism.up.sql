-- Add up migration script here
ALTER TABLE store_galleries
ADD COLUMN gallery_type VARCHAR(255) NOT NULL,
ADD COLUMN gallery_type_id VARCHAR(255) NOT NULL;

CREATE INDEX idx_store_galleries_gallery_type_id ON store_galleries (gallery_type_id);
CREATE INDEX idx_store_galleries_gallery_type ON store_galleries (gallery_type);