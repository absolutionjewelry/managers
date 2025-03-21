-- Add down migration script here
DROP INDEX IF EXISTS idx_postable_reactions_user_id;
DROP INDEX IF EXISTS idx_postable_reactions_postable_type;
DROP INDEX IF EXISTS idx_postable_reactions_postable_id;
DROP INDEX IF EXISTS idx_postable_reactions_reaction_type_id;
DROP INDEX IF EXISTS idx_postable_reactions_created_at;
DROP INDEX IF EXISTS idx_postable_reactions_archived_at;
DROP TABLE IF EXISTS postable_reactions;

DROP INDEX IF EXISTS idx_reaction_types_archived_at;
DROP INDEX IF EXISTS idx_reaction_types_updated_at;
DROP INDEX IF EXISTS idx_reaction_types_created_at;
DROP INDEX IF EXISTS idx_reaction_types_reaction_name;
DROP INDEX IF EXISTS idx_reaction_types_id;
DROP TABLE IF EXISTS reaction_types;

