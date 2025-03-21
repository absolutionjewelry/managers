-- Add down migration script here
DROP INDEX IF EXISTS idx_taggables_created_at;
DROP INDEX IF EXISTS idx_taggables_taggable_type;
DROP INDEX IF EXISTS idx_taggables_taggable_id;
DROP INDEX IF EXISTS idx_taggables_tag_id;
COMMENT ON COLUMN taggables.taggable_type IS NULL;
DROP TABLE IF EXISTS taggables;

DROP INDEX IF EXISTS idx_tags_archived_at;
DROP INDEX IF EXISTS idx_tags_created_at;
DROP INDEX IF EXISTS idx_tags_tag_name;
DROP TABLE IF EXISTS tags;