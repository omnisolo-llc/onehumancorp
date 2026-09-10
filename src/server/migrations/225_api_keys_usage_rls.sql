-- +goose Up
-- Migration 225: Enable missing RLS for api_keys and user_usage_logs

-- Add tenant_id if it doesn't exist, and set a default to prevent application INSERT failures.
ALTER TABLE IF EXISTS api_keys ADD COLUMN IF NOT EXISTS tenant_id UUID DEFAULT (current_setting('app.current_tenant', true)::uuid);
UPDATE api_keys SET tenant_id = organization_id::UUID WHERE tenant_id IS NULL;

ALTER TABLE IF EXISTS user_usage_logs ADD COLUMN IF NOT EXISTS tenant_id UUID DEFAULT (current_setting('app.current_tenant', true)::uuid);
UPDATE user_usage_logs SET tenant_id = organization_id::UUID WHERE tenant_id IS NULL;

ALTER TABLE IF EXISTS api_keys ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS api_keys FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_api_keys ON api_keys;
CREATE POLICY tenant_isolation_api_keys ON api_keys
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

ALTER TABLE IF EXISTS user_usage_logs ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS user_usage_logs FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_user_usage_logs ON user_usage_logs;
CREATE POLICY tenant_isolation_user_usage_logs ON user_usage_logs
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

GRANT ALL PRIVILEGES ON api_keys, user_usage_logs TO ohc_bypassrls;
