-- Add up migration script here
CREATE TABLE IF NOT EXISTS store_products (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    product_name VARCHAR(255) NOT NULL,
    product_description TEXT,
    product_base_cost DECIMAL(10, 2) NOT NULL,
    product_base_price DECIMAL(10, 2) NOT NULL,
    product_base_quantity INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP,
    UNIQUE (store_id, product_name),
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_products_store_id ON store_products (store_id);
CREATE INDEX IF NOT EXISTS idx_store_products_product_name ON store_products (product_name);
CREATE INDEX IF NOT EXISTS idx_store_products_product_base_cost ON store_products (product_base_cost);
CREATE INDEX IF NOT EXISTS idx_store_products_product_base_price ON store_products (product_base_price);
CREATE INDEX IF NOT EXISTS idx_store_products_product_base_quantity ON store_products (product_base_quantity);
CREATE INDEX IF NOT EXISTS idx_store_products_created_at ON store_products (created_at);
CREATE INDEX IF NOT EXISTS idx_store_products_updated_at ON store_products (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_products_archived_at ON store_products (archived_at);

CREATE TABLE IF NOT EXISTS store_product_variants (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    variant_name VARCHAR(255) NOT NULL,
    variant_description TEXT,
    variant_base_cost DECIMAL(10, 2) NOT NULL,
    variant_base_price DECIMAL(10, 2) NOT NULL,
    variant_base_quantity INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP,
    UNIQUE (store_id, variant_name),
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_product_variants_store_id ON store_product_variants (store_id);
CREATE INDEX IF NOT EXISTS idx_store_product_variants_variant_name ON store_product_variants (variant_name);
CREATE INDEX IF NOT EXISTS idx_store_product_variants_variant_base_cost ON store_product_variants (variant_base_cost);
CREATE INDEX IF NOT EXISTS idx_store_product_variants_variant_base_price ON store_product_variants (variant_base_price);
CREATE INDEX IF NOT EXISTS idx_store_product_variants_variant_base_quantity ON store_product_variants (variant_base_quantity);
CREATE INDEX IF NOT EXISTS idx_store_product_variants_created_at ON store_product_variants (created_at);
CREATE INDEX IF NOT EXISTS idx_store_product_variants_updated_at ON store_product_variants (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_product_variants_archived_at ON store_product_variants (archived_at);

CREATE TABLE IF NOT EXISTS store_products_variants (
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_product_id VARCHAR(255) NOT NULL REFERENCES store_products(id),
    store_product_variant_id VARCHAR(255) NOT NULL REFERENCES store_product_variants(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (store_id, store_product_id, store_product_variant_id),
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_product_id) REFERENCES store_products(id),
    FOREIGN KEY (store_product_variant_id) REFERENCES store_product_variants(id)
);

CREATE INDEX IF NOT EXISTS idx_store_products_variants_store_id ON store_products_variants (store_id);
CREATE INDEX IF NOT EXISTS idx_store_products_variants_store_product_id ON store_products_variants (store_product_id);
CREATE INDEX IF NOT EXISTS idx_store_products_variants_store_product_variant_id ON store_products_variants (store_product_variant_id);
CREATE INDEX IF NOT EXISTS idx_store_products_variants_created_at ON store_products_variants (created_at);
