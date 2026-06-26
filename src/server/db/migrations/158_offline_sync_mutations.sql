CREATE TABLE IF NOT EXISTS offline_sync_mutations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    table_name TEXT NOT NULL,
    operation TEXT NOT NULL,
    payload JSONB NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    idempotency_key TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_offline_sync_mutations_tenant_idempotency
ON offline_sync_mutations(tenant_id, idempotency_key);

CREATE INDEX IF NOT EXISTS idx_offline_sync_mutations_status
ON offline_sync_mutations(status);

ALTER TABLE offline_sync_mutations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_offline_sync_mutations ON offline_sync_mutations;
CREATE POLICY tenant_isolation_offline_sync_mutations
ON offline_sync_mutations
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
