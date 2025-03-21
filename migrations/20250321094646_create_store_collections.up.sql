-- Add up migration script here
CREATE TABLE IF NOT EXISTS store_collections (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    collection_name VARCHAR(255) NOT NULL,
    collection_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_collections_store_id ON store_collections (store_id);
CREATE INDEX IF NOT EXISTS idx_store_collections_collection_name ON store_collections (collection_name);
CREATE INDEX IF NOT EXISTS idx_store_collections_created_at ON store_collections (created_at);
CREATE INDEX IF NOT EXISTS idx_store_collections_updated_at ON store_collections (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_collections_archived_at ON store_collections (archived_at);

CREATE TABLE IF NOT EXISTS store_collections_products (
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_collection_id VARCHAR(255) NOT NULL REFERENCES store_collections(id),
    store_product_id VARCHAR(255) NOT NULL REFERENCES store_products(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (store_id, store_collection_id, store_product_id),
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_collection_id) REFERENCES store_collections(id),
    FOREIGN KEY (store_product_id) REFERENCES store_products(id)
);

CREATE INDEX IF NOT EXISTS idx_store_collections_products_store_id ON store_collections_products (store_id);
CREATE INDEX IF NOT EXISTS idx_store_collections_products_store_collection_id ON store_collections_products (store_collection_id);
CREATE INDEX IF NOT EXISTS idx_store_collections_products_store_product_id ON store_collections_products (store_product_id);
CREATE INDEX IF NOT EXISTS idx_store_collections_products_created_at ON store_collections_products (created_at);
