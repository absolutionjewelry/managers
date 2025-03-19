-- Add down migration script here
DROP TABLE IF EXISTS users;
DROP INDEX IF EXISTS idx_users_first_name;
DROP INDEX IF EXISTS idx_users_last_name;
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_users_user_password;
DROP INDEX IF EXISTS idx_users_created_at;
DROP INDEX IF EXISTS idx_users_updated_at;