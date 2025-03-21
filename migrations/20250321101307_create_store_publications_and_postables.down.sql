-- Add down migration script here
DROP INDEX IF EXISTS idx_store_publication_postables_published_at;
DROP INDEX IF EXISTS idx_store_publication_postables_archived_at;
DROP INDEX IF EXISTS idx_store_publication_postables_updated_at;
DROP INDEX IF EXISTS idx_store_publication_postables_created_at;
DROP INDEX IF EXISTS idx_store_publication_postables_store_publication_id;
DROP INDEX IF EXISTS idx_store_publication_postables_store_id;
DROP INDEX IF EXISTS idx_store_publication_postables_user_id;
DROP INDEX IF EXISTS idx_store_publication_postables_postable_type;
DROP INDEX IF EXISTS idx_store_publication_postables_id;
DROP TABLE IF EXISTS store_publication_postables;

DROP INDEX IF EXISTS idx_store_publications_archived_at;
DROP INDEX IF EXISTS idx_store_publications_updated_at;
DROP INDEX IF EXISTS idx_store_publications_created_at;
DROP INDEX IF EXISTS idx_store_publications_store_id;
DROP INDEX IF EXISTS idx_store_publications_id;
DROP TABLE IF EXISTS store_publications;