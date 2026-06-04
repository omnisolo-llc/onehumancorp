-- Create shared_tasks table as required by #3977
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    dependencies JSONB DEFAULT '[]',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_organization_id ON shared_tasks(organization_id);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_status ON shared_tasks(status);
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));