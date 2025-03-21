-- Add down migration script here
DROP INDEX IF EXISTS idx_store_users_store_id;
DROP INDEX IF EXISTS idx_store_users_user_id;
DROP INDEX IF EXISTS idx_store_users_created_at;
DROP INDEX IF EXISTS idx_store_users_updated_at;
DROP INDEX IF EXISTS idx_store_users_archived_at;
DROP TABLE IF EXISTS store_users;

DROP INDEX IF EXISTS idx_store_roles_store_id;
DROP INDEX IF EXISTS idx_store_roles_id;
DROP INDEX IF EXISTS idx_store_roles_role_name;
DROP INDEX IF EXISTS idx_store_roles_role_description;
DROP INDEX IF EXISTS idx_store_roles_created_at;
DROP INDEX IF EXISTS idx_store_roles_updated_at;
DROP INDEX IF EXISTS idx_store_roles_archived_at;
DROP TABLE IF EXISTS store_roles;

DROP INDEX IF EXISTS idx_stores_owner_id;
DROP INDEX IF EXISTS idx_stores_store_name;
DROP INDEX IF EXISTS idx_stores_store_description;
DROP INDEX IF EXISTS idx_stores_created_at;
DROP INDEX IF EXISTS idx_stores_updated_at;
DROP INDEX IF EXISTS idx_stores_archived_at;
DROP TABLE IF EXISTS stores;