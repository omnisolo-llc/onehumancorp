ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
CREATE INDEX IF NOT EXISTS idx_swarm_tasks_tenant_id ON swarm_tasks(tenant_id);
ALTER TABLE task_dependencies ADD COLUMN IF NOT EXISTS tenant_id TEXT;
CREATE INDEX IF NOT EXISTS idx_task_dependencies_tenant_id ON task_dependencies(tenant_id);
ALTER TABLE shared_task_dependencies ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_shared_task_dependencies_org_id ON shared_task_dependencies(organization_id);
