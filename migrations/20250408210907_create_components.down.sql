-- Add down migration script here
DROP INDEX IF EXISTS idx_componentables_id;
DROP INDEX IF EXISTS idx_componentables_created_at;
DROP INDEX IF EXISTS idx_componentables_updated_at;
DROP INDEX IF EXISTS idx_componentables_component_id;
DROP INDEX IF EXISTS idx_componentables_componentable_id;
DROP INDEX IF EXISTS idx_componentables_componentable_type;

DROP TABLE IF EXISTS componentables;

DROP INDEX IF EXISTS idx_components_component_description;
DROP INDEX IF EXISTS idx_components_cost_per_unit;
DROP INDEX IF EXISTS idx_components_quantity;
DROP INDEX IF EXISTS idx_components_unit_of_measure;
DROP INDEX IF EXISTS idx_components_component_name;
DROP INDEX IF EXISTS idx_components_id;
DROP INDEX IF EXISTS idx_components_archived_at;
DROP INDEX IF EXISTS idx_components_created_at;
DROP INDEX IF EXISTS idx_components_updated_at;

DROP TABLE IF EXISTS components;