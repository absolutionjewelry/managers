-- Add up migration script here
CREATE TABLE IF NOT EXISTS favoriteables (
    id VARCHAR(255) PRIMARY KEY,
    favoriteable_type VARCHAR(255) NOT NULL,
    favoriteable_id VARCHAR(255) NOT NULL REFERENCES store_publication_postables(id),
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (favoriteable_id) REFERENCES store_publication_postables(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_favoriteables_id ON favoriteables (id);
CREATE INDEX IF NOT EXISTS idx_favoriteables_favoriteable_id ON favoriteables (favoriteable_id);
CREATE INDEX IF NOT EXISTS idx_favoriteables_favoriteable_type ON favoriteables (favoriteable_type);
CREATE INDEX IF NOT EXISTS idx_favoriteables_user_id ON favoriteables (user_id);
CREATE INDEX IF NOT EXISTS idx_favoriteables_created_at ON favoriteables (created_at);
CREATE INDEX IF NOT EXISTS idx_favoriteables_archived_at ON favoriteables (archived_at);
