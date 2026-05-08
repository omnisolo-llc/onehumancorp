-- 073_missing_rls_policies_for_revoked_tokens.sql

ALTER TABLE revoked_tokens ADD COLUMN IF NOT EXISTS organization_id TEXT DEFAULT 'system';
ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_revoked_tokens_strict ON revoked_tokens USING (organization_id::text = current_setting('app.current_tenant', true));
