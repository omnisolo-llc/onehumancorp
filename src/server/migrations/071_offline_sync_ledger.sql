CREATE TABLE IF NOT EXISTS offline_sync_ledger (
    transaction_id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    quantity_deducted INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE offline_sync_ledger ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_offline_sync_ledger ON offline_sync_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
