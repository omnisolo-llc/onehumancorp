-- 059_ultraplan_rls_policies.sql

-- Drop the overly permissive missing RLS if any, just make sure we are clean
-- (no such policies created yet in harden)

-- For swarm_ultra_plans
ALTER TABLE swarm_ultra_plans ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE swarm_ultra_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_ultra_plans ON swarm_ultra_plans;
CREATE POLICY tenant_isolation_swarm_ultra_plans ON swarm_ultra_plans USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- For swarm_dream_epochs
ALTER TABLE swarm_dream_epochs ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE swarm_dream_epochs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_dream_epochs ON swarm_dream_epochs;
CREATE POLICY tenant_isolation_swarm_dream_epochs ON swarm_dream_epochs USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- For sub_agent_jobs
ALTER TABLE sub_agent_jobs ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE sub_agent_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sub_agent_jobs ON sub_agent_jobs;
CREATE POLICY tenant_isolation_sub_agent_jobs ON sub_agent_jobs USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- For state_machine_transitions
ALTER TABLE state_machine_transitions ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- For shared_tasks_decomposition
-- (It already has organization_id column)
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
