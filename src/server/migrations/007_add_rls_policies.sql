-- Migration: 007_add_rls_policies.sql

ALTER TABLE IF EXISTS agent_kv_store ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_kv_store ON agent_kv_store USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS department_dead_letters ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_department_dead_letters ON department_dead_letters USING (tenant_id::text = current_setting('app.current_tenant', true));
