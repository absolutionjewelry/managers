-- Add down migration script here
DROP TABLE IF EXISTS tenant_users;
DROP TABLE IF EXISTS tenant_roles;
DROP TABLE IF EXISTS tenants;

DROP INDEX IF EXISTS idx_tenant_users_tenant_id;
DROP INDEX IF EXISTS idx_tenant_users_user_id;
DROP INDEX IF EXISTS idx_tenant_users_created_at;
DROP INDEX IF EXISTS idx_tenant_users_updated_at;
DROP INDEX IF EXISTS idx_tenant_users_archived_at;

DROP INDEX IF EXISTS idx_tenant_roles_tenant_id;
DROP INDEX IF EXISTS idx_tenant_roles_id;
DROP INDEX IF EXISTS idx_tenant_roles_role_name;
DROP INDEX IF EXISTS idx_tenant_roles_role_description;
DROP INDEX IF EXISTS idx_tenant_roles_created_at;

DROP INDEX IF EXISTS idx_tenants_owner_id;
DROP INDEX IF EXISTS idx_tenants_tenant_name;
DROP INDEX IF EXISTS idx_tenants_tenant_description;
DROP INDEX IF EXISTS idx_tenants_created_at;
DROP INDEX IF EXISTS idx_tenants_updated_at;
DROP INDEX IF EXISTS idx_tenants_archived_at;
