-- Add down migration script here
DROP INDEX IF EXISTS idx_favoriteables_archived_at;
DROP INDEX IF EXISTS idx_favoriteables_created_at;
DROP INDEX IF EXISTS idx_favoriteables_user_id;
DROP INDEX IF EXISTS idx_favoriteables_favoriteable_type;
DROP INDEX IF EXISTS idx_favoriteables_favoriteable_id;
DROP INDEX IF EXISTS idx_favoriteables_id;
DROP TABLE IF EXISTS favoriteables;
