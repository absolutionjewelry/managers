-- Add up migration script here
ALTER TABLE store_products
ALTER COLUMN product_base_quantity TYPE FLOAT USING product_base_quantity::FLOAT;