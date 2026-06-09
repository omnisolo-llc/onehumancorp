CREATE TABLE IF NOT EXISTS field_service_jobs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT,
    customer_name TEXT NOT NULL,
    service_requested TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'IN_PROGRESS', 'COMPLETED', 'CANCELLED')),
    notes TEXT,
    scheduled_at TIMESTAMPTZ NOT NULL,
    location TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_field_service_jobs_tenant ON field_service_jobs(tenant_id, scheduled_at);

ALTER TABLE field_service_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS field_service_jobs_tenant_isolation ON field_service_jobs;
CREATE POLICY field_service_jobs_tenant_isolation ON field_service_jobs
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
