ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (tenant_id::text = current_setting('app.current_tenant', true));
