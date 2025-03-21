-- Add up migration script here
CREATE TABLE IF NOT EXISTS contact_modes (
    id VARCHAR(255) PRIMARY KEY,
    contact_mode_name VARCHAR(255) NOT NULL,
    contact_mode_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);

CREATE INDEX IF NOT EXISTS idx_contact_modes_id ON contact_modes (id);
CREATE INDEX IF NOT EXISTS idx_contact_modes_contact_mode_name ON contact_modes (contact_mode_name);
CREATE INDEX IF NOT EXISTS idx_contact_modes_created_at ON contact_modes (created_at);
CREATE INDEX IF NOT EXISTS idx_contact_modes_updated_at ON contact_modes (updated_at);
CREATE INDEX IF NOT EXISTS idx_contact_modes_archived_at ON contact_modes (archived_at);

CREATE TABLE IF NOT EXISTS user_contact_modes (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    contact_mode_id VARCHAR(255) NOT NULL REFERENCES contact_modes(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (contact_mode_id) REFERENCES contact_modes(id)
);

CREATE INDEX IF NOT EXISTS idx_user_contact_modes_id ON user_contact_modes (id);
CREATE INDEX IF NOT EXISTS idx_user_contact_modes_user_id ON user_contact_modes (user_id);
CREATE INDEX IF NOT EXISTS idx_user_contact_modes_contact_mode_id ON user_contact_modes (contact_mode_id);
CREATE INDEX IF NOT EXISTS idx_user_contact_modes_created_at ON user_contact_modes (created_at);
CREATE INDEX IF NOT EXISTS idx_user_contact_modes_updated_at ON user_contact_modes (updated_at);
CREATE INDEX IF NOT EXISTS idx_user_contact_modes_archived_at ON user_contact_modes (archived_at);