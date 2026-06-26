-- Migration 162: Edge Ledger Transactions for Tap-to-Pay offline mode
CREATE TABLE IF NOT EXISTS edge_ledger_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL DEFAULT 'PENDING', -- PENDING, SYNCED, RECONCILED, FAILED
    device_signature TEXT,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    synced_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_edge_ledger_tenant_id_transaction_id ON edge_ledger_transactions(tenant_id, transaction_id);
CREATE INDEX IF NOT EXISTS idx_edge_ledger_status ON edge_ledger_transactions(status);

ALTER TABLE edge_ledger_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_edge_ledger_transactions ON edge_ledger_transactions;
CREATE POLICY tenant_isolation_edge_ledger_transactions
ON edge_ledger_transactions
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
