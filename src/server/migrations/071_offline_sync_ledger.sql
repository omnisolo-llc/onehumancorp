CREATE TABLE IF NOT EXISTS synced_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    transaction_id VARCHAR NOT NULL,
    product_id VARCHAR NOT NULL,
    quantity_deducted INT NOT NULL,
    timestamp TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, transaction_id)
);
