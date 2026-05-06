-- 069_final_rls_policies.sql
-- Ensure all remaining tables with tenant_id or organization_id are strictly protected with RLS

ALTER TABLE department_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_department_tasks ON department_tasks;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
DROP POLICY IF EXISTS tenant_isolation_tasks_strict ON tasks;

CREATE POLICY tenant_isolation_department_tasks ON department_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_tasks_strict ON tasks USING (organization_id::text = current_setting('app.current_tenant', true));
