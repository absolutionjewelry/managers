-- Add up migration script here
CREATE TABLE IF NOT EXISTS store_user_carts (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_carts_id ON store_user_carts (id);
CREATE INDEX IF NOT EXISTS idx_store_user_carts_store_id ON store_user_carts (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_carts_user_id ON store_user_carts (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_carts_created_at ON store_user_carts (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_carts_updated_at ON store_user_carts (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_carts_archived_at ON store_user_carts (archived_at);

CREATE TABLE IF NOT EXISTS store_user_cart_products (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_cart_id VARCHAR(255) NOT NULL REFERENCES store_user_carts(id),
    store_product_id VARCHAR(255) NOT NULL REFERENCES store_products(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    quantity INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_cart_id) REFERENCES store_user_carts(id),
    FOREIGN KEY (store_product_id) REFERENCES store_products(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_cart_products_id ON store_user_cart_products (id);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_products_store_id ON store_user_cart_products (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_products_store_user_cart_id ON store_user_cart_products (store_user_cart_id);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_products_store_product_id ON store_user_cart_products (store_product_id);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_products_user_id ON store_user_cart_products (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_products_quantity ON store_user_cart_products (quantity);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_products_created_at ON store_user_cart_products (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_products_updated_at ON store_user_cart_products (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_products_archived_at ON store_user_cart_products (archived_at);

CREATE TABLE IF NOT EXISTS store_user_cart_product_variations (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_cart_product_id VARCHAR(255) NOT NULL REFERENCES store_user_cart_products(id),
    store_product_variation_id VARCHAR(255) NOT NULL REFERENCES store_product_variants(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    quantity INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_cart_product_id) REFERENCES store_user_cart_products(id),
    FOREIGN KEY (store_product_variation_id) REFERENCES store_product_variants(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_cart_product_variations_id ON store_user_cart_product_variations (id);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_product_variations_store_id ON store_user_cart_product_variations (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_product_variations_store_user_cart_product_id ON store_user_cart_product_variations (store_user_cart_product_id);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_product_variations_store_product_variation_id ON store_user_cart_product_variations (store_product_variation_id);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_product_variations_user_id ON store_user_cart_product_variations (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_product_variations_quantity ON store_user_cart_product_variations (quantity);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_product_variations_created_at ON store_user_cart_product_variations (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_product_variations_updated_at ON store_user_cart_product_variations (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_cart_product_variations_archived_at ON store_user_cart_product_variations (archived_at);

CREATE TABLE IF NOT EXISTS store_user_product_saves (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_cart_id VARCHAR(255) NOT NULL REFERENCES store_user_carts(id),
    store_product_id VARCHAR(255) NOT NULL REFERENCES store_products(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    quantity INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_cart_id) REFERENCES store_user_carts(id),
    FOREIGN KEY (store_product_id) REFERENCES store_products(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_product_saves_id ON store_user_product_saves (id);
CREATE INDEX IF NOT EXISTS idx_store_user_product_saves_store_id ON store_user_product_saves (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_product_saves_store_user_cart_id ON store_user_product_saves (store_user_cart_id);
CREATE INDEX IF NOT EXISTS idx_store_user_product_saves_store_product_id ON store_user_product_saves (store_product_id);
CREATE INDEX IF NOT EXISTS idx_store_user_product_saves_user_id ON store_user_product_saves (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_product_saves_quantity ON store_user_product_saves (quantity);
CREATE INDEX IF NOT EXISTS idx_store_user_product_saves_created_at ON store_user_product_saves (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_product_saves_updated_at ON store_user_product_saves (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_product_saves_archived_at ON store_user_product_saves (archived_at);

CREATE TABLE IF NOT EXISTS store_user_product_variation_saves (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_product_save_id VARCHAR(255) NOT NULL REFERENCES store_user_product_saves(id),
    store_product_variation_id VARCHAR(255) NOT NULL REFERENCES store_product_variants(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    quantity INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_product_save_id) REFERENCES store_user_product_saves(id),
    FOREIGN KEY (store_product_variation_id) REFERENCES store_product_variants(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_product_variation_saves_id ON store_user_product_variation_saves (id);
CREATE INDEX IF NOT EXISTS idx_store_user_product_variation_saves_store_id ON store_user_product_variation_saves (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_product_variation_saves_store_user_product_save_id ON store_user_product_variation_saves (store_user_product_save_id);
CREATE INDEX IF NOT EXISTS idx_store_user_product_variation_saves_store_product_variation_id ON store_user_product_variation_saves (store_product_variation_id);
CREATE INDEX IF NOT EXISTS idx_store_user_product_variation_saves_user_id ON store_user_product_variation_saves (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_product_variation_saves_quantity ON store_user_product_variation_saves (quantity);
CREATE INDEX IF NOT EXISTS idx_store_user_product_variation_saves_created_at ON store_user_product_variation_saves (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_product_variation_saves_updated_at ON store_user_product_variation_saves (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_product_variation_saves_archived_at ON store_user_product_variation_saves (archived_at);