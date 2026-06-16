CREATE TABLE IF NOT EXISTS agent_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    job_type VARCHAR NOT NULL,
    payload JSONB NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'pending',
    attempts INT NOT NULL DEFAULT 0,
    max_attempts INT NOT NULL DEFAULT 3,
    run_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    locked_at TIMESTAMPTZ,
    locked_by VARCHAR
);

CREATE INDEX IF NOT EXISTS idx_agent_jobs_pickup ON agent_jobs (status, run_at) WHERE status = 'pending';

-- Enforce RLS
ALTER TABLE agent_jobs ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'agent_jobs'
            AND policyname = 'tenant_isolation_agent_jobs'
    ) THEN
        CREATE POLICY tenant_isolation_agent_jobs ON agent_jobs
        USING (tenant_id = current_setting('app.current_tenant', true))
        WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
    END IF;
END
$$;
