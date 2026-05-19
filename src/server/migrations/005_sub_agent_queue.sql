CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    parent_task_id UUID NOT NULL,
    payload JSONB,
    status VARCHAR NOT NULL DEFAULT 'QUEUED', -- QUEUED, RUNNING, COMPLETED, FAILED
    worker_id VARCHAR,
    attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 3,
    run_after TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE sub_agent_queue ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'sub_agent_queue'
            AND policyname = 'tenant_isolation_sub_agent_queue'
    ) THEN
        CREATE POLICY tenant_isolation_sub_agent_queue
            ON sub_agent_queue
            USING (organization_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
