-- +goose Up
-- Apply missing tenant_id and RLS to tables

-- epics
ALTER TABLE IF EXISTS epics ADD COLUMN IF NOT EXISTS tenant_id UUID DEFAULT (current_setting('app.current_tenant', true)::uuid);
ALTER TABLE IF EXISTS epics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_policy ON epics;
CREATE POLICY tenant_isolation_policy ON epics USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- tasks
ALTER TABLE IF EXISTS tasks ADD COLUMN IF NOT EXISTS tenant_id UUID DEFAULT (current_setting('app.current_tenant', true)::uuid);
ALTER TABLE IF EXISTS tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_policy ON tasks;
CREATE POLICY tenant_isolation_policy ON tasks USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- task_dependencies
ALTER TABLE IF EXISTS task_dependencies ADD COLUMN IF NOT EXISTS tenant_id UUID DEFAULT (current_setting('app.current_tenant', true)::uuid);
ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_policy ON task_dependencies;
CREATE POLICY tenant_isolation_policy ON task_dependencies USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- shared_task_dependencies
ALTER TABLE IF EXISTS shared_task_dependencies ADD COLUMN IF NOT EXISTS tenant_id UUID DEFAULT (current_setting('app.current_tenant', true)::uuid);
ALTER TABLE IF EXISTS shared_task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_policy ON shared_task_dependencies;
CREATE POLICY tenant_isolation_policy ON shared_task_dependencies USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- embedding_cache
ALTER TABLE IF EXISTS embedding_cache ADD COLUMN IF NOT EXISTS tenant_id UUID DEFAULT (current_setting('app.current_tenant', true)::uuid);
ALTER TABLE IF EXISTS embedding_cache ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_policy ON embedding_cache;
CREATE POLICY tenant_isolation_policy ON embedding_cache USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- telemetry_buffer
ALTER TABLE IF EXISTS telemetry_buffer ADD COLUMN IF NOT EXISTS tenant_id UUID DEFAULT (current_setting('app.current_tenant', true)::uuid);
ALTER TABLE IF EXISTS telemetry_buffer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_policy ON telemetry_buffer;
CREATE POLICY tenant_isolation_policy ON telemetry_buffer USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- api_keys
ALTER TABLE IF EXISTS api_keys ADD COLUMN IF NOT EXISTS tenant_id UUID DEFAULT (current_setting('app.current_tenant', true)::uuid);
ALTER TABLE IF EXISTS api_keys ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_policy ON api_keys;
CREATE POLICY tenant_isolation_policy ON api_keys USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- user_usage_logs
ALTER TABLE IF EXISTS user_usage_logs ADD COLUMN IF NOT EXISTS tenant_id UUID DEFAULT (current_setting('app.current_tenant', true)::uuid);
ALTER TABLE IF EXISTS user_usage_logs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_policy ON user_usage_logs;
CREATE POLICY tenant_isolation_policy ON user_usage_logs USING (tenant_id = current_setting('app.current_tenant', true)::uuid);
