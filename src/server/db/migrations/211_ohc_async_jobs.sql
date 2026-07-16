CREATE TABLE IF NOT EXISTS ohc_async_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    job_type VARCHAR NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    retry_count INT NOT NULL DEFAULT 0,
    max_retries INT NOT NULL DEFAULT 3,
    next_retry_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_async_jobs_polling_next_retry ON ohc_async_jobs(next_retry_at) WHERE status = 'PENDING';
CREATE INDEX IF NOT EXISTS idx_ohc_async_jobs_polling_type ON ohc_async_jobs(job_type) WHERE status = 'PENDING';
CREATE INDEX IF NOT EXISTS idx_ohc_async_jobs_tenant_status ON ohc_async_jobs(tenant_id, status);

ALTER TABLE ohc_async_jobs ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'ohc_async_jobs'
            AND policyname = 'tenant_isolation_ohc_async_jobs'
    ) THEN
        CREATE POLICY tenant_isolation_ohc_async_jobs ON ohc_async_jobs
        USING (tenant_id::text = current_setting('app.current_tenant', true))
        WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
