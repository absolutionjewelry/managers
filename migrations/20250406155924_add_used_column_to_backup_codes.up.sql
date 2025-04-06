-- Add up migration script here
ALTER TABLE backup_codes ADD COLUMN used BOOLEAN NOT NULL DEFAULT FALSE;
CREATE INDEX idx_backup_codes_used ON backup_codes (used);