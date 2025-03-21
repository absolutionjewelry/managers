-- Add up migration script here
CREATE TABLE IF NOT EXISTS store_roles_users (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    role_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (role_id) REFERENCES store_roles(id)
);

CREATE INDEX IF NOT EXISTS idx_store_roles_users_store_id ON store_roles_users(store_id);
CREATE INDEX IF NOT EXISTS idx_store_roles_users_user_id ON store_roles_users(user_id);
CREATE INDEX IF NOT EXISTS idx_store_roles_users_role_id ON store_roles_users(role_id);
CREATE INDEX IF NOT EXISTS idx_store_roles_users_created_at ON store_roles_users(created_at);
CREATE INDEX IF NOT EXISTS idx_store_roles_users_updated_at ON store_roles_users(updated_at);
CREATE INDEX IF NOT EXISTS idx_store_roles_users_archived_at ON store_roles_users(archived_at);