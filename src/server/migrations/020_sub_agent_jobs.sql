CREATE TABLE IF NOT EXISTS sub_agent_jobs (
    id TEXT PRIMARY KEY,
    parent_task_id TEXT,
    agent_role TEXT NOT NULL,
    payload JSONB,
    status TEXT NOT NULL DEFAULT 'QUEUED',
    attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 3,
    run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    locked_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sub_agent_jobs_org_status_run_after ON sub_agent_jobs(organization_id, status, run_after);
CREATE INDEX IF NOT EXISTS idx_sub_agent_jobs_role_status_run_after ON sub_agent_jobs(agent_role, status, run_after);

ALTER TABLE sub_agent_jobs ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_sub_agent_jobs ON sub_agent_jobs USING (organization_id::text = current_setting('app.current_tenant', true));
