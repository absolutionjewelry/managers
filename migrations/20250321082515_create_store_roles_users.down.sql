-- Add down migration script here
DROP TABLE IF EXISTS store_roles_users;

DROP INDEX IF EXISTS idx_store_roles_users_store_id;
DROP INDEX IF EXISTS idx_store_roles_users_user_id;
DROP INDEX IF EXISTS idx_store_roles_users_role_id;
DROP INDEX IF EXISTS idx_store_roles_users_created_at;
DROP INDEX IF EXISTS idx_store_roles_users_updated_at;
DROP INDEX IF EXISTS idx_store_roles_users_archived_at;
