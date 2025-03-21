-- Add up migration script here
CREATE TABLE IF NOT EXISTS store_user_orders (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_cart_id VARCHAR(255) NOT NULL REFERENCES store_user_carts(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_cart_id) REFERENCES store_user_carts(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_orders_id ON store_user_orders (id);
CREATE INDEX IF NOT EXISTS idx_store_user_orders_store_id ON store_user_orders (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_orders_store_user_cart_id ON store_user_orders (store_user_cart_id);
CREATE INDEX IF NOT EXISTS idx_store_user_orders_user_id ON store_user_orders (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_orders_created_at ON store_user_orders (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_orders_updated_at ON store_user_orders (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_orders_archived_at ON store_user_orders (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_products (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_order_id VARCHAR(255) NOT NULL REFERENCES store_user_orders(id),
    store_product_id VARCHAR(255) NOT NULL REFERENCES store_products(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    quantity INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_order_id) REFERENCES store_user_orders(id),
    FOREIGN KEY (store_product_id) REFERENCES store_products(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_products_id ON store_user_order_products (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_products_store_id ON store_user_order_products (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_products_store_user_order_id ON store_user_order_products (store_user_order_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_products_store_product_id ON store_user_order_products (store_product_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_products_user_id ON store_user_order_products (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_products_quantity ON store_user_order_products (quantity);
CREATE INDEX IF NOT EXISTS idx_store_user_order_products_created_at ON store_user_order_products (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_products_updated_at ON store_user_order_products (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_products_archived_at ON store_user_order_products (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_product_variations (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_order_id VARCHAR(255) NOT NULL REFERENCES store_user_orders(id),
    store_product_variation_id VARCHAR(255) NOT NULL REFERENCES store_product_variants(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    quantity INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_order_id) REFERENCES store_user_orders(id),
    FOREIGN KEY (store_product_variation_id) REFERENCES store_product_variants(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_product_variations_id ON store_user_order_product_variations (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_product_variations_store_id ON store_user_order_product_variations (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_product_variations_store_user_order_id ON store_user_order_product_variations (store_user_order_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_product_variations_store_product_variation_id ON store_user_order_product_variations (store_product_variation_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_product_variations_user_id ON store_user_order_product_variations (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_product_variations_quantity ON store_user_order_product_variations (quantity);
CREATE INDEX IF NOT EXISTS idx_store_user_order_product_variations_created_at ON store_user_order_product_variations (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_product_variations_updated_at ON store_user_order_product_variations (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_product_variations_archived_at ON store_user_order_product_variations (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_comments (
    id VARCHAR(255) PRIMARY KEY,
    parent_id VARCHAR(255) REFERENCES store_user_order_comments(id),
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_order_id VARCHAR(255) NOT NULL REFERENCES store_user_orders(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    comment TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_order_id) REFERENCES store_user_orders(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_comments_id ON store_user_order_comments (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_comments_parent_id ON store_user_order_comments (parent_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_comments_store_id ON store_user_order_comments (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_comments_store_user_order_id ON store_user_order_comments (store_user_order_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_comments_user_id ON store_user_order_comments (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_comments_created_at ON store_user_order_comments (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_comments_updated_at ON store_user_order_comments (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_comments_archived_at ON store_user_order_comments (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_status_types (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    status_type_name VARCHAR(255) NOT NULL,
    status_type_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_status_types_id ON store_user_order_status_types (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_status_types_store_id ON store_user_order_status_types (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_status_types_status_type_name ON store_user_order_status_types (status_type_name);
CREATE INDEX IF NOT EXISTS idx_store_user_order_status_types_created_at ON store_user_order_status_types (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_status_types_updated_at ON store_user_order_status_types (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_status_types_archived_at ON store_user_order_status_types (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_statuses (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_order_id VARCHAR(255) NOT NULL REFERENCES store_user_orders(id),
    status_type_id VARCHAR(255) NOT NULL REFERENCES store_user_order_status_types(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_order_id) REFERENCES store_user_orders(id),
    FOREIGN KEY (status_type_id) REFERENCES store_user_order_status_types(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_statuses_id ON store_user_order_statuses (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_statuses_store_id ON store_user_order_statuses (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_statuses_store_user_order_id ON store_user_order_statuses (store_user_order_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_statuses_status_type_id ON store_user_order_statuses (status_type_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_statuses_user_id ON store_user_order_statuses (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_statuses_created_at ON store_user_order_statuses (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_statuses_updated_at ON store_user_order_statuses (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_statuses_archived_at ON store_user_order_statuses (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_notes (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_order_id VARCHAR(255) NOT NULL REFERENCES store_user_orders(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    note TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_order_id) REFERENCES store_user_orders(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_notes_id ON store_user_order_notes (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_notes_store_id ON store_user_order_notes (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_notes_store_user_order_id ON store_user_order_notes (store_user_order_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_notes_user_id ON store_user_order_notes (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_notes_created_at ON store_user_order_notes (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_notes_updated_at ON store_user_order_notes (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_notes_archived_at ON store_user_order_notes (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_payment_types (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    payment_type_name VARCHAR(255) NOT NULL,
    payment_type_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_types_id ON store_user_order_payment_types (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_types_store_id ON store_user_order_payment_types (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_types_payment_type_name ON store_user_order_payment_types (payment_type_name);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_types_created_at ON store_user_order_payment_types (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_types_updated_at ON store_user_order_payment_types (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_types_archived_at ON store_user_order_payment_types (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_payment_status_types (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    payment_status_type_name VARCHAR(255) NOT NULL,
    payment_status_type_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_status_types_id ON store_user_order_payment_status_types (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_status_types_store_id ON store_user_order_payment_status_types (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_status_types_payment_status_type_name ON store_user_order_payment_status_types (payment_status_type_name);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_status_types_created_at ON store_user_order_payment_status_types (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_status_types_updated_at ON store_user_order_payment_status_types (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payment_status_types_archived_at ON store_user_order_payment_status_types (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_payments (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_order_id VARCHAR(255) NOT NULL REFERENCES store_user_orders(id),
    payment_type_id VARCHAR(255) NOT NULL REFERENCES store_user_order_payment_types(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    payment_amount DECIMAL(10, 2) NOT NULL,
    payment_status_type_id VARCHAR(255) NOT NULL REFERENCES store_user_order_payment_status_types(id),
    payment_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    payment_reference VARCHAR(255) NOT NULL,
    payment_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_order_id) REFERENCES store_user_orders(id),
    FOREIGN KEY (payment_type_id) REFERENCES store_user_order_payment_types(id),
    FOREIGN KEY (payment_status_type_id) REFERENCES store_user_order_payment_status_types(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_payments_id ON store_user_order_payments (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payments_store_id ON store_user_order_payments (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payments_store_user_order_id ON store_user_order_payments (store_user_order_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payments_payment_type_id ON store_user_order_payments (payment_type_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payments_user_id ON store_user_order_payments (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payments_payment_status_type_id ON store_user_order_payments (payment_status_type_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payments_created_at ON store_user_order_payments (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payments_updated_at ON store_user_order_payments (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_payments_archived_at ON store_user_order_payments (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_assignment_status_types (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    assignment_status_type_name VARCHAR(255) NOT NULL,
    assignment_status_type_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_assignment_status_types_id ON store_user_order_assignment_status_types (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignment_status_types_store_id ON store_user_order_assignment_status_types (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignment_status_types_assignment_status_type_name ON store_user_order_assignment_status_types (assignment_status_type_name);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignment_status_types_created_at ON store_user_order_assignment_status_types (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignment_status_types_updated_at ON store_user_order_assignment_status_types (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignment_status_types_archived_at ON store_user_order_assignment_status_types (archived_at);

CREATE TABLE IF NOT EXISTS store_user_order_assignments (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_user_order_id VARCHAR(255) NOT NULL REFERENCES store_user_orders(id),
    assignment_status_type_id VARCHAR(255) NOT NULL REFERENCES store_user_order_assignment_status_types(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    assigned_id VARCHAR(255) NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_user_order_id) REFERENCES store_user_orders(id),
    FOREIGN KEY (assignment_status_type_id) REFERENCES store_user_order_assignment_status_types(id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (assigned_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_user_order_assignments_id ON store_user_order_assignments (id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignments_store_id ON store_user_order_assignments (store_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignments_store_user_order_id ON store_user_order_assignments (store_user_order_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignments_user_id ON store_user_order_assignments (user_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignments_assigned_id ON store_user_order_assignments (assigned_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignments_assignment_status_type_id ON store_user_order_assignments (assignment_status_type_id);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignments_created_at ON store_user_order_assignments (created_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignments_updated_at ON store_user_order_assignments (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_user_order_assignments_archived_at ON store_user_order_assignments (archived_at);