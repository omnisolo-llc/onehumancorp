ALTER TABLE swarm_tasks ADD COLUMN tenant_id TEXT NOT NULL DEFAULT 'default_tenant';
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));