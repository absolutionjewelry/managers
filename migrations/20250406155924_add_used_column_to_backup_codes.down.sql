-- Add down migration script here
DROP INDEX idx_backup_codes_used;
ALTER TABLE backup_codes DROP COLUMN used;