-- Add down migration script here
DROP TABLE IF EXISTS authentications;
DROP INDEX IF EXISTS idx_authentications_user_id;
DROP INDEX IF EXISTS idx_authentications_token;
DROP INDEX IF EXISTS idx_authentications_expires_at;
DROP INDEX IF EXISTS idx_authentications_created_at;
DROP INDEX IF EXISTS idx_authentications_archived_at;
