CREATE TABLE IF NOT EXISTS synced_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    transaction_id VARCHAR NOT NULL,
    product_id VARCHAR NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, transaction_id)
);

ALTER TABLE synced_transactions ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy ON synced_transactions
    USING (tenant_id = current_setting('app.current_tenant', true));
