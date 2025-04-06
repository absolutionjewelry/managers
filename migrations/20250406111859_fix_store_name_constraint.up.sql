-- Add up migration script here
ALTER TABLE stores
DROP CONSTRAINT stores_store_name_key;

ALTER TABLE stores
ADD CONSTRAINT unique_store_name UNIQUE (store_name, owner_id, archived_at);
