-- 048_kairos_shared_tasks_schema.sql
-- Create shared_tasks schema for KAIROS Orchestration

CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    agent_id TEXT,
    priority TEXT NOT NULL DEFAULT 'P2',
    payload JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id = current_setting('app.current_tenant', true));
