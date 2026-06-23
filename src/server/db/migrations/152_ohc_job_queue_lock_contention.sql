-- High-Performance Agentic Background Job Queue lock contention fix
-- GitHub Issue #30463

-- Add indices for typical queue polling to reduce lock contention and sequential scans
CREATE INDEX IF NOT EXISTS idx_ohc_job_queue_polling_next_retry ON ohc_job_queue(next_retry_at) WHERE status = 'PENDING';
CREATE INDEX IF NOT EXISTS idx_ohc_job_queue_polling_type ON ohc_job_queue(job_type) WHERE status = 'PENDING';

-- Make sure RLS is properly enforced to make multi-tenant polling secure
ALTER TABLE ohc_job_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_job_queue ON ohc_job_queue;
CREATE POLICY tenant_isolation_ohc_job_queue ON ohc_job_queue USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
