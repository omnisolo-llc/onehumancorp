CREATE TABLE IF NOT EXISTS offline_sync_ledger (
    transaction_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity_deducted INT NOT NULL,
    synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE offline_sync_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_offline_sync_ledger ON offline_sync_ledger;
CREATE POLICY tenant_isolation_offline_sync_ledger ON offline_sync_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
