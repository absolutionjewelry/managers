-- Add up migration script here
ALTER TABLE store_products
ALTER COLUMN product_base_price TYPE FLOAT USING product_base_price::FLOAT;

ALTER TABLE store_products
ALTER COLUMN product_base_cost TYPE FLOAT USING product_base_cost::FLOAT;

