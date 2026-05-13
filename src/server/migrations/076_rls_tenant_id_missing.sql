-- 076_rls_tenant_id_missing.sql
-- Add missing RLS policy for revoked_tokens and roles which are global but missing tenant isolation
ALTER TABLE revoked_tokens ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE roles ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';

ALTER TABLE IF EXISTS revoked_tokens ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS roles ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_revoked_tokens_strict ON revoked_tokens;
CREATE POLICY tenant_isolation_revoked_tokens_strict ON revoked_tokens USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_roles_strict ON roles;
CREATE POLICY tenant_isolation_roles_strict ON roles USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
