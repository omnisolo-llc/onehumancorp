ALTER TABLE api_keys ADD COLUMN IF NOT EXISTS tenant_id VARCHAR(128);
UPDATE api_keys SET tenant_id = organization_id WHERE tenant_id IS NULL;
ALTER TABLE api_keys ALTER COLUMN tenant_id SET NOT NULL;
ALTER TABLE api_keys ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_api_keys ON api_keys USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE user_usage_logs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR(128);
UPDATE user_usage_logs SET tenant_id = organization_id WHERE tenant_id IS NULL;
ALTER TABLE user_usage_logs ALTER COLUMN tenant_id SET NOT NULL;
ALTER TABLE user_usage_logs ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_user_usage_logs ON user_usage_logs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
