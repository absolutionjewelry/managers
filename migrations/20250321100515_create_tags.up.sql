-- Add up migration script here
CREATE TABLE IF NOT EXISTS tags (
    id VARCHAR(255) PRIMARY KEY,
    tag_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);

CREATE INDEX IF NOT EXISTS idx_tags_tag_name ON tags (tag_name);
CREATE INDEX IF NOT EXISTS idx_tags_created_at ON tags (created_at);
CREATE INDEX IF NOT EXISTS idx_tags_archived_at ON tags (archived_at);

CREATE TABLE IF NOT EXISTS taggables (
    tag_id VARCHAR(255) NOT NULL REFERENCES tags(id),
    taggable_id VARCHAR(255) NOT NULL,
    taggable_type VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (tag_id, taggable_id, taggable_type),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

COMMENT ON COLUMN taggables.taggable_type IS 'Types available: "store_product", "store_product_variant", "store_collection", "store_gallery", "etc."';
CREATE INDEX IF NOT EXISTS idx_taggables_tag_id ON taggables (tag_id);
CREATE INDEX IF NOT EXISTS idx_taggables_taggable_id ON taggables (taggable_id);
CREATE INDEX IF NOT EXISTS idx_taggables_taggable_type ON taggables (taggable_type);
CREATE INDEX IF NOT EXISTS idx_taggables_created_at ON taggables (created_at);