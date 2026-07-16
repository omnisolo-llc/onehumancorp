-- Multi-Tenant Async Queue
CREATE TABLE IF NOT EXISTS ohc_async_jobs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    status TEXT NOT NULL DEFAULT 'PENDING',
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    next_retry_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_async_jobs_polling ON ohc_async_jobs(status, next_retry_at) WHERE status = 'PENDING';
CREATE INDEX IF NOT EXISTS idx_ohc_async_jobs_tenant ON ohc_async_jobs(tenant_id);

ALTER TABLE ohc_async_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_async_jobs ON ohc_async_jobs;
CREATE POLICY tenant_isolation_ohc_async_jobs
ON ohc_async_jobs
USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
