-- Add down migration script here
ALTER TABLE store_products
ALTER COLUMN product_base_price TYPE DECIMAL USING product_base_price::DECIMAL;

ALTER TABLE store_products
ALTER COLUMN product_base_cost TYPE DECIMAL USING product_base_cost::DECIMAL;

