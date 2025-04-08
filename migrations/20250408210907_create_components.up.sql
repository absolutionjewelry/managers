-- Add up migration script here
CREATE TABLE IF NOT EXISTS components (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    component_name VARCHAR(255) NOT NULL,
    component_description TEXT,
    cost_per_unit FLOAT NOT NULL,
    quantity FLOAT NOT NULL,
    unit_of_measure VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_components_component_name ON components (component_name);
CREATE UNIQUE INDEX IF NOT EXISTS idx_components_id ON components (id);
CREATE INDEX IF NOT EXISTS idx_components_archived_at ON components (archived_at);
CREATE INDEX IF NOT EXISTS idx_components_created_at ON components (created_at);
CREATE INDEX IF NOT EXISTS idx_components_updated_at ON components (updated_at);
CREATE INDEX IF NOT EXISTS idx_components_cost_per_unit ON components (cost_per_unit);
CREATE INDEX IF NOT EXISTS idx_components_quantity ON components (quantity);
CREATE INDEX IF NOT EXISTS idx_components_unit_of_measure ON components (unit_of_measure);
CREATE INDEX IF NOT EXISTS idx_components_component_description ON components (component_description);

CREATE TABLE IF NOT EXISTS componentables (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    component_id VARCHAR(255) NOT NULL,
    componentable_id VARCHAR(255) NOT NULL,
    componentable_type VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_componentables_id ON componentables (id);
CREATE INDEX IF NOT EXISTS idx_componentables_created_at ON componentables (created_at);
CREATE INDEX IF NOT EXISTS idx_componentables_updated_at ON componentables (updated_at);
CREATE INDEX IF NOT EXISTS idx_componentables_component_id ON componentables (component_id);
CREATE INDEX IF NOT EXISTS idx_componentables_componentable_id ON componentables (componentable_id);
CREATE INDEX IF NOT EXISTS idx_componentables_componentable_type ON componentables (componentable_type);
