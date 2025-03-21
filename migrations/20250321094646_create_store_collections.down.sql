-- Add down migration script here
DROP INDEX IF EXISTS idx_store_collections_products_store_product_id;
DROP INDEX IF EXISTS idx_store_collections_products_store_collection_id;
DROP INDEX IF EXISTS idx_store_collections_products_store_id;
DROP INDEX IF EXISTS idx_store_collections_products_created_at;
DROP TABLE IF EXISTS store_collections_products;

DROP INDEX IF EXISTS idx_store_collections_archived_at;
DROP INDEX IF EXISTS idx_store_collections_updated_at;
DROP INDEX IF EXISTS idx_store_collections_created_at;
DROP INDEX IF EXISTS idx_store_collections_collection_name;
DROP INDEX IF EXISTS idx_store_collections_store_id;
DROP TABLE IF EXISTS store_collections;
