-- Add up migration script here
CREATE TABLE IF NOT EXISTS store_images (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    image_content_type VARCHAR(255) NOT NULL,
    image_content_length INT NOT NULL,
    image_content BYTEA NOT NULL,
    image_name VARCHAR(255) NOT NULL,
    image_description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_images_store_id ON store_images (store_id);
CREATE INDEX IF NOT EXISTS idx_store_images_image_content_type ON store_images (image_content_type);
CREATE INDEX IF NOT EXISTS idx_store_images_image_content_length ON store_images (image_content_length);
CREATE INDEX IF NOT EXISTS idx_store_images_image_name ON store_images (image_name);
CREATE INDEX IF NOT EXISTS idx_store_images_created_at ON store_images (created_at);
CREATE INDEX IF NOT EXISTS idx_store_images_updated_at ON store_images (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_images_archived_at ON store_images (archived_at);

CREATE TABLE IF NOT EXISTS store_galleries (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    gallery_name VARCHAR(255) NOT NULL,
    gallery_description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_galleries_store_id ON store_galleries (store_id);
CREATE INDEX IF NOT EXISTS idx_store_galleries_gallery_name ON store_galleries (gallery_name);
CREATE INDEX IF NOT EXISTS idx_store_galleries_created_at ON store_galleries (created_at);
CREATE INDEX IF NOT EXISTS idx_store_galleries_updated_at ON store_galleries (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_galleries_archived_at ON store_galleries (archived_at);

CREATE TABLE IF NOT EXISTS store_galleries_images (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_gallery_id VARCHAR(255) NOT NULL REFERENCES store_galleries(id),
    store_gallery_type VARCHAR(255) NOT NULL,
    store_gallery_position INT NOT NULL DEFAULT 0,
    store_image_id VARCHAR(255) NOT NULL REFERENCES store_images(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_gallery_id) REFERENCES store_galleries(id),
    FOREIGN KEY (store_image_id) REFERENCES store_images(id)
);

COMMENT ON COLUMN store_galleries_images.store_gallery_type IS 'Types available: "store", "store_product", "store_product_variant", "etc."';

CREATE INDEX IF NOT EXISTS idx_store_galleries_images_store_id ON store_galleries_images (store_id);
CREATE INDEX IF NOT EXISTS idx_store_galleries_images_store_gallery_id ON store_galleries_images (store_gallery_id);
CREATE INDEX IF NOT EXISTS idx_store_galleries_images_store_gallery_type ON store_galleries_images (store_gallery_type);
CREATE INDEX IF NOT EXISTS idx_store_galleries_images_store_gallery_position ON store_galleries_images (store_gallery_position);
CREATE INDEX IF NOT EXISTS idx_store_galleries_images_created_at ON store_galleries_images (created_at);
