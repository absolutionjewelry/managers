-- Add down migration script here
DROP INDEX IF EXISTS idx_user_contact_modes_archived_at;
DROP INDEX IF EXISTS idx_user_contact_modes_updated_at;
DROP INDEX IF EXISTS idx_user_contact_modes_created_at;
DROP INDEX IF EXISTS idx_user_contact_modes_contact_mode_id;
DROP INDEX IF EXISTS idx_user_contact_modes_user_id;
DROP INDEX IF EXISTS idx_user_contact_modes_id;
DROP TABLE IF EXISTS user_contact_modes;

DROP INDEX IF EXISTS idx_contact_modes_archived_at;
DROP INDEX IF EXISTS idx_contact_modes_updated_at;
DROP INDEX IF EXISTS idx_contact_modes_created_at;
DROP INDEX IF EXISTS idx_contact_modes_contact_mode_name;
DROP INDEX IF EXISTS idx_contact_modes_id;
DROP TABLE IF EXISTS contact_modes;