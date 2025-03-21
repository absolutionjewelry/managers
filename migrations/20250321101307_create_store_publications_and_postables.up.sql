-- Add up migration script here
CREATE TABLE IF NOT EXISTS store_publications (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    publication_name VARCHAR(255) NOT NULL,
    publication_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_publications_id ON store_publications (id);
CREATE INDEX IF NOT EXISTS idx_store_publications_store_id ON store_publications (store_id);
CREATE INDEX IF NOT EXISTS idx_store_publications_publication_name ON store_publications (publication_name);
CREATE INDEX IF NOT EXISTS idx_store_publications_created_at ON store_publications (created_at);
CREATE INDEX IF NOT EXISTS idx_store_publications_updated_at ON store_publications (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_publications_archived_at ON store_publications (archived_at);

CREATE TABLE IF NOT EXISTS store_publication_postables (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL REFERENCES stores(id),
    store_publication_id VARCHAR(255) NOT NULL REFERENCES store_publications(id),
    postable_type VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    postable_name VARCHAR(255) NOT NULL,
    postable_description TEXT,
    postable_content TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    published_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (store_publication_id) REFERENCES store_publications(id)
);

CREATE INDEX IF NOT EXISTS idx_store_publication_postables_id ON store_publication_postables (id);
CREATE INDEX IF NOT EXISTS idx_store_publication_postables_store_id ON store_publication_postables (store_id);
CREATE INDEX IF NOT EXISTS idx_store_publication_postables_store_publication_id ON store_publication_postables (store_publication_id);
CREATE INDEX IF NOT EXISTS idx_store_publication_postables_postable_name ON store_publication_postables (postable_name);
CREATE INDEX IF NOT EXISTS idx_store_publication_postables_user_id ON store_publication_postables (user_id);
CREATE INDEX IF NOT EXISTS idx_store_publication_postables_postable_type ON store_publication_postables (postable_type);
CREATE INDEX IF NOT EXISTS idx_store_publication_postables_created_at ON store_publication_postables (created_at);
CREATE INDEX IF NOT EXISTS idx_store_publication_postables_updated_at ON store_publication_postables (updated_at);
CREATE INDEX IF NOT EXISTS idx_store_publication_postables_archived_at ON store_publication_postables (archived_at);
CREATE INDEX IF NOT EXISTS idx_store_publication_postables_published_at ON store_publication_postables (published_at);