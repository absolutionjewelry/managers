-- Add down migration script here
DROP INDEX idx_store_user_orders_order_stage_id;
ALTER TABLE store_user_orders
DROP COLUMN order_stage_id;

DROP TABLE store_user_order_status_stage_types;
