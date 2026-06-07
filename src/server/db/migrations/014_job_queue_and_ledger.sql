-- High-Performance Background Job Queue and Universal Ledger
-- GitHub Issue #22405
CREATE TABLE IF NOT EXISTS ohc_job_queue (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    parent_task_id TEXT,
    job_type TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    status TEXT NOT NULL DEFAULT 'PENDING', -- PENDING, PROCESSING, COMPLETED, FAILED
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    next_retry_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    locked_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_ohc_job_queue_polling
ON ohc_job_queue(status, next_retry_at)
WHERE status = 'PENDING';
CREATE INDEX IF NOT EXISTS idx_ohc_job_queue_tenant
ON ohc_job_queue(tenant_id);
ALTER TABLE ohc_job_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_job_queue ON ohc_job_queue;
CREATE POLICY tenant_isolation_ohc_job_queue
ON ohc_job_queue
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE TABLE IF NOT EXISTS ohc_universal_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    department TEXT NOT NULL,
    action_type TEXT NOT NULL,
    state_change JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_ohc_universal_ledger_tenant
ON ohc_universal_ledger(tenant_id, created_at DESC);
ALTER TABLE ohc_universal_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger;
CREATE POLICY tenant_isolation_ohc_universal_ledger
ON ohc_universal_ledger
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Implement append-only constraint via trigger
CREATE OR REPLACE FUNCTION prevent_ledger_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'ohc_universal_ledger is append-only';
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS trg_append_only_ledger_update ON ohc_universal_ledger;
CREATE TRIGGER trg_append_only_ledger_update
BEFORE UPDATE ON ohc_universal_ledger
FOR EACH ROW EXECUTE FUNCTION prevent_ledger_update_or_delete();
DROP TRIGGER IF EXISTS trg_append_only_ledger_delete ON ohc_universal_ledger;
CREATE TRIGGER trg_append_only_ledger_delete
BEFORE DELETE ON ohc_universal_ledger
FOR EACH ROW EXECUTE FUNCTION prevent_ledger_update_or_delete();