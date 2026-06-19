CREATE TABLE IF NOT EXISTS swarm_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL DEFAULT 'default_tenant',
    mission_id TEXT NOT NULL,
    parent_plan_id TEXT,
    dependencies JSONB NOT NULL DEFAULT '[]',
    title TEXT NOT NULL,
    description TEXT,
    priority TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    locked_until TIMESTAMPTZ,
    payload JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    auto_dreamed BOOLEAN DEFAULT FALSE,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
