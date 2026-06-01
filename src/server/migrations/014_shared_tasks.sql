-- Drop the existing shared_tasks table if it exists as we're recreating it
DROP TABLE IF EXISTS shared_tasks CASCADE;

CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    mission_id TEXT NOT NULL DEFAULT '',
    parent_plan_id TEXT NOT NULL DEFAULT '',
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    priority TEXT NOT NULL DEFAULT 'NORMAL',
    payload TEXT NOT NULL DEFAULT '',
    dependencies JSONB DEFAULT '[]',
    locked_until TIMESTAMPTZ,
    ultraplan_phase TEXT,
    deliberation_log JSONB DEFAULT '[]',
    depth INTEGER,
    action_risk TEXT,
    approval_status TEXT,
    proposed_content TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    -- Columns to support tenant isolation and queues correctly in KAIROS
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1,
    auto_dreamed BOOLEAN DEFAULT FALSE
);

CREATE INDEX IF NOT EXISTS idx_shared_tasks_organization_id ON shared_tasks(organization_id);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_status ON shared_tasks(status);

ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
