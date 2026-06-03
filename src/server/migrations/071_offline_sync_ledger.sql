CREATE TABLE IF NOT EXISTS offline_sync_ledger (
    transaction_id VARCHAR(255) PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    product_id VARCHAR(255) NOT NULL,
    quantity_deducted INT NOT NULL,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE offline_sync_ledger ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_offline_sync_ledger ON offline_sync_ledger USING (tenant_id = current_setting('app.current_tenant_id', true));
