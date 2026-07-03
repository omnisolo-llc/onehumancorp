ALTER TABLE IF EXISTS shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
