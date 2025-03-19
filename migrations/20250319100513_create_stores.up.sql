-- Add up migration script here
CREATE TABLE IF NOT EXISTS stores (
    id VARCHAR(255) PRIMARY KEY,
    owner_id VARCHAR(255) NOT NULL,
    store_name VARCHAR(255) NOT NULL UNIQUE,
    store_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (owner_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_stores_owner_id ON stores(owner_id);
CREATE INDEX IF NOT EXISTS idx_stores_store_name ON stores(store_name);
CREATE INDEX IF NOT EXISTS idx_stores_store_description ON stores(store_description);
CREATE INDEX IF NOT EXISTS idx_stores_created_at ON stores(created_at);
CREATE INDEX IF NOT EXISTS idx_stores_updated_at ON stores(updated_at);
CREATE INDEX IF NOT EXISTS idx_stores_archived_at ON stores(archived_at);

CREATE TABLE IF NOT EXISTS store_users (
    id VARCHAR(255) PRIMARY KEY,
    store_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_store_users_store_id ON store_users(store_id);
CREATE INDEX IF NOT EXISTS idx_store_users_user_id ON store_users(user_id);
CREATE INDEX IF NOT EXISTS idx_store_users_created_at ON store_users(created_at);
CREATE INDEX IF NOT EXISTS idx_store_users_updated_at ON store_users(updated_at);
CREATE INDEX IF NOT EXISTS idx_store_users_archived_at ON store_users(archived_at);

CREATE TABLE IF NOT EXISTS store_roles (
    store_id VARCHAR(255) NOT NULL,
    id VARCHAR(255) PRIMARY KEY,
    role_name VARCHAR(255) NOT NULL,
    role_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (store_id) REFERENCES stores(id)
);

CREATE INDEX IF NOT EXISTS idx_store_roles_store_id ON store_roles(store_id);
CREATE INDEX IF NOT EXISTS idx_store_roles_id ON store_roles(id);
CREATE INDEX IF NOT EXISTS idx_store_roles_role_name ON store_roles(role_name);
CREATE INDEX IF NOT EXISTS idx_store_roles_role_description ON store_roles(role_description);
CREATE INDEX IF NOT EXISTS idx_store_roles_created_at ON store_roles(created_at);
