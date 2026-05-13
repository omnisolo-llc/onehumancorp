-- Enforce Multi-Tenant Isolation
ALTER TABLE agents ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agents ON agents USING (organization_id = current_setting('app.current_tenant', true));

ALTER TABLE agent_memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_memory ON agent_memory USING (organization_id = current_setting('app.current_tenant', true));

ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tasks ON tasks USING (organization_id = current_setting('app.current_tenant', true));
