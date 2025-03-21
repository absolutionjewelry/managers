-- Add up migration script here
CREATE TABLE IF NOT EXISTS reaction_types (
    id VARCHAR(255) PRIMARY KEY,
    reaction_name VARCHAR(255) NOT NULL,
    reaction_description TEXT,
    reaction_image TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);

CREATE INDEX IF NOT EXISTS idx_reaction_types_id ON reaction_types (id);
CREATE INDEX IF NOT EXISTS idx_reaction_types_reaction_name ON reaction_types (reaction_name);
CREATE INDEX IF NOT EXISTS idx_reaction_types_created_at ON reaction_types (created_at);
CREATE INDEX IF NOT EXISTS idx_reaction_types_updated_at ON reaction_types (updated_at);
CREATE INDEX IF NOT EXISTS idx_reaction_types_archived_at ON reaction_types (archived_at);

CREATE TABLE IF NOT EXISTS postable_reactions (
    id VARCHAR(255) PRIMARY KEY,
    postable_type VARCHAR(255) NOT NULL,
    postable_id VARCHAR(255) NOT NULL REFERENCES store_publication_postables(id),
    reaction_type_id VARCHAR(255) NOT NULL REFERENCES reaction_types(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (postable_id) REFERENCES store_publication_postables(id),
    FOREIGN KEY (reaction_type_id) REFERENCES reaction_types(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_postable_reactions_postable_id ON postable_reactions (postable_id);
CREATE INDEX IF NOT EXISTS idx_postable_reactions_postable_type ON postable_reactions (postable_type);
CREATE INDEX IF NOT EXISTS idx_postable_reactions_reaction_type_id ON postable_reactions (reaction_type_id);
CREATE INDEX IF NOT EXISTS idx_postable_reactions_user_id ON postable_reactions (user_id);
CREATE INDEX IF NOT EXISTS idx_postable_reactions_created_at ON postable_reactions (created_at);
CREATE INDEX IF NOT EXISTS idx_postable_reactions_archived_at ON postable_reactions (archived_at);
