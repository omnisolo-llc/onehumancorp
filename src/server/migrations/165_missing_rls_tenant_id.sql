-- +goose Up
-- Add tenant_id and RLS to tables that were missing them

ALTER TABLE task_dependencies ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;

-- Default tenant_id to 'system' to prevent null constraints failures on existing data, or handle gracefully
UPDATE task_dependencies SET tenant_id = 'system' WHERE tenant_id IS NULL;
UPDATE agent_session_data SET tenant_id = 'system' WHERE tenant_id IS NULL;
UPDATE swarm_truth_embeddings SET tenant_id = 'system' WHERE tenant_id IS NULL;
UPDATE swarm_tasks SET tenant_id = 'system' WHERE tenant_id IS NULL;

-- Enable RLS and add policies
ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_session_data ON agent_session_data;
CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings;
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
ALTER TABLE task_dependencies DISABLE ROW LEVEL SECURITY;
ALTER TABLE task_dependencies DROP COLUMN IF EXISTS tenant_id;

DROP POLICY IF EXISTS tenant_isolation_agent_session_data ON agent_session_data;
ALTER TABLE agent_session_data DISABLE ROW LEVEL SECURITY;
ALTER TABLE agent_session_data DROP COLUMN IF EXISTS tenant_id;

DROP POLICY IF EXISTS tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings;
ALTER TABLE swarm_truth_embeddings DISABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_truth_embeddings DROP COLUMN IF EXISTS tenant_id;

DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
ALTER TABLE swarm_tasks DISABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_tasks DROP COLUMN IF EXISTS tenant_id;
