CREATE TABLE IF NOT EXISTS localized_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    original_amount REAL NOT NULL,
    original_currency TEXT NOT NULL,
    target_currency TEXT NOT NULL,
    applied_fx_rate REAL NOT NULL,
    applied_margin REAL NOT NULL,
    final_amount REAL NOT NULL,
    is_offline BOOLEAN NOT NULL DEFAULT FALSE,
    reconciled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_localized_transactions_tenant
ON localized_transactions(tenant_id);

ALTER TABLE localized_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_localized_transactions ON localized_transactions;
CREATE POLICY tenant_isolation_localized_transactions
ON localized_transactions
USING (tenant_id = current_setting('app.current_tenant', true));
