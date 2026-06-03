CREATE TABLE IF NOT EXISTS offline_sync_transactions (
    transaction_id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    quantity_deducted INT NOT NULL,
    synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE offline_sync_transactions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_offline_sync_transactions ON offline_sync_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
