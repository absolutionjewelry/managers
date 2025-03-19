-- Add up migration script here
CREATE TABLE IF NOT EXISTS tenants (
    id VARCHAR(255) PRIMARY KEY,
    owner_id VARCHAR(255) NOT NULL,
    tenant_name VARCHAR(255) NOT NULL UNIQUE,
    tenant_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (owner_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_tenants_owner_id ON tenants(owner_id);
CREATE INDEX IF NOT EXISTS idx_tenants_tenant_name ON tenants(tenant_name);
CREATE INDEX IF NOT EXISTS idx_tenants_tenant_description ON tenants(tenant_description);
CREATE INDEX IF NOT EXISTS idx_tenants_created_at ON tenants(created_at);
CREATE INDEX IF NOT EXISTS idx_tenants_updated_at ON tenants(updated_at);
CREATE INDEX IF NOT EXISTS idx_tenants_archived_at ON tenants(archived_at);

CREATE TABLE IF NOT EXISTS tenant_users (
    id VARCHAR(255) PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (tenant_id) REFERENCES tenants(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_tenant_users_tenant_id ON tenant_users(tenant_id);
CREATE INDEX IF NOT EXISTS idx_tenant_users_user_id ON tenant_users(user_id);
CREATE INDEX IF NOT EXISTS idx_tenant_users_created_at ON tenant_users(created_at);
CREATE INDEX IF NOT EXISTS idx_tenant_users_updated_at ON tenant_users(updated_at);
CREATE INDEX IF NOT EXISTS idx_tenant_users_archived_at ON tenant_users(archived_at);

CREATE TABLE IF NOT EXISTS tenant_roles (
    tenant_id VARCHAR(255) NOT NULL,
    id VARCHAR(255) PRIMARY KEY,
    role_name VARCHAR(255) NOT NULL,
    role_description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    archived_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    FOREIGN KEY (tenant_id) REFERENCES tenants(id)
);

CREATE INDEX IF NOT EXISTS idx_tenant_roles_tenant_id ON tenant_roles(tenant_id);
CREATE INDEX IF NOT EXISTS idx_tenant_roles_id ON tenant_roles(id);
CREATE INDEX IF NOT EXISTS idx_tenant_roles_role_name ON tenant_roles(role_name);
CREATE INDEX IF NOT EXISTS idx_tenant_roles_role_description ON tenant_roles(role_description);
CREATE INDEX IF NOT EXISTS idx_tenant_roles_created_at ON tenant_roles(created_at);
