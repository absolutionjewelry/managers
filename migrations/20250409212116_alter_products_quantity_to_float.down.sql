-- Add down migration script here
ALTER TABLE store_products
ALTER COLUMN product_base_quantity TYPE INT USING product_base_quantity::INT;
