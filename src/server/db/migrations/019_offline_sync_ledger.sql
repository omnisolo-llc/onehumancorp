CREATE TABLE IF NOT EXISTS synced_transactions (
    transaction_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity_deducted INT NOT NULL,
    timestamp TIMESTAMPTZ,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_synced_transactions_tenant
ON synced_transactions(tenant_id);

ALTER TABLE synced_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_synced_transactions ON synced_transactions;
CREATE POLICY tenant_isolation_synced_transactions ON synced_transactions USING (tenant_id = current_setting('app.current_tenant', true));
