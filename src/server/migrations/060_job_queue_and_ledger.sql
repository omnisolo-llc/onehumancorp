-- Create ohc_job_queue table
CREATE TABLE IF NOT EXISTS ohc_job_queue (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    job_type TEXT NOT NULL,
    payload JSONB DEFAULT '{}'::jsonb,
    status TEXT NOT NULL DEFAULT 'PENDING',
    retry_count INTEGER DEFAULT 0,
    next_retry_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_job_queue_tenant_status ON ohc_job_queue(tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_ohc_job_queue_next_retry ON ohc_job_queue(status, next_retry_at);

ALTER TABLE ohc_job_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_job_queue ON ohc_job_queue;
CREATE POLICY tenant_isolation_ohc_job_queue ON ohc_job_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- Create ohc_universal_ledger table
CREATE TABLE IF NOT EXISTS ohc_universal_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    department TEXT NOT NULL,
    payload JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_universal_ledger_tenant ON ohc_universal_ledger(tenant_id, created_at);
CREATE INDEX IF NOT EXISTS idx_ohc_universal_ledger_dept ON ohc_universal_ledger(tenant_id, department);

ALTER TABLE ohc_universal_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger;
CREATE POLICY tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
