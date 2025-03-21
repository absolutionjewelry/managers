-- Add up migration script here
CREATE TABLE store_user_order_status_stage_types (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP,
    UNIQUE (store_id, name)
);

CREATE INDEX idx_store_user_order_status_stage_types_store_id ON store_user_order_status_stage_types(store_id);
CREATE INDEX idx_store_user_order_status_stage_types_name ON store_user_order_status_stage_types(name);
CREATE INDEX idx_store_user_order_status_stage_types_created_at ON store_user_order_status_stage_types(created_at);
CREATE INDEX idx_store_user_order_status_stage_types_updated_at ON store_user_order_status_stage_types(updated_at);
CREATE INDEX idx_store_user_order_status_stage_types_archived_at ON store_user_order_status_stage_types(archived_at);

ALTER TABLE store_user_orders
ADD COLUMN order_stage_id VARCHAR(255) NOT NULL REFERENCES store_user_order_status_stage_types(id);

CREATE INDEX idx_store_user_orders_order_stage_id ON store_user_orders(order_stage_id);
