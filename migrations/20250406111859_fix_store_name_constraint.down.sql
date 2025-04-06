-- Add down migration script here
ALTER TABLE stores
DROP CONSTRAINT unique_store_name;

ALTER TABLE stores
ADD CONSTRAINT stores_store_name_key UNIQUE (store_name);

