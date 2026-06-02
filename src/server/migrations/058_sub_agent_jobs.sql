-- Migration 058: Implement Missing sub_agent_jobs Database Schema

CREATE TABLE IF NOT EXISTS sub_agent_jobs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    parent_task_id TEXT NOT NULL,
    agent_role TEXT NOT NULL,
    payload TEXT,
    status TEXT NOT NULL DEFAULT 'QUEUED', -- QUEUED, RUNNING, COMPLETED, FAILED
    attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 3,
    run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    locked_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_sub_agent_jobs_tenant_id ON sub_agent_jobs(tenant_id);
CREATE INDEX IF NOT EXISTS idx_sub_agent_jobs_status_run_after ON sub_agent_jobs(status, run_after);

-- Enable Row Level Security (RLS) for tenant isolation
ALTER TABLE sub_agent_jobs ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'sub_agent_jobs'
            AND policyname = 'tenant_isolation_sub_agent_jobs'
    ) THEN
        CREATE POLICY tenant_isolation_sub_agent_jobs ON sub_agent_jobs
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
