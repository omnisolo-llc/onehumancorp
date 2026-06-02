-- Migration 058: Create sub_agent_jobs table

CREATE TABLE IF NOT EXISTS sub_agent_jobs (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    parent_task_id VARCHAR NOT NULL,
    agent_role VARCHAR NOT NULL,
    payload JSONB,
    status VARCHAR NOT NULL DEFAULT 'QUEUED',
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    run_after TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    locked_until TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

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
        CREATE POLICY tenant_isolation_sub_agent_jobs
            ON sub_agent_jobs
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
