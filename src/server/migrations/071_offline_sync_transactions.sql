CREATE TABLE IF NOT EXISTS synced_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE synced_transactions ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenant_isolation_synced_transactions"
ON synced_transactions
FOR ALL
USING (tenant_id = current_setting('app.current_tenant_id', true));
