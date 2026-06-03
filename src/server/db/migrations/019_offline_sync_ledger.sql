-- Migration to add synced_transactions table for offline sync idempotency
-- GitHub Issue #23415

CREATE TABLE IF NOT EXISTS synced_transactions (
    transaction_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity_deducted INTEGER NOT NULL,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (transaction_id, tenant_id)
);

CREATE INDEX IF NOT EXISTS idx_synced_transactions_tenant_product
ON synced_transactions(tenant_id, product_id);

ALTER TABLE synced_transactions ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_synced_transactions ON synced_transactions;
CREATE POLICY tenant_isolation_synced_transactions
ON synced_transactions
USING (tenant_id = current_setting('app.current_tenant', true));
