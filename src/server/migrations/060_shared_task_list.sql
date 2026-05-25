CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload TEXT,
    parent_plan_id TEXT,
    dependencies TEXT NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (organization_id::text = current_setting('app.current_tenant', true));
