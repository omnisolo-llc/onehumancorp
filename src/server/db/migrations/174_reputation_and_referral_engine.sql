-- +goose Up
CREATE TABLE IF NOT EXISTS universal_wallet_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    credit_amount BIGINT NOT NULL,
    reason TEXT NOT NULL,
    created_at_unix BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_universal_wallet_ledger_tenant ON universal_wallet_ledger(tenant_id);
CREATE INDEX IF NOT EXISTS idx_universal_wallet_ledger_customer ON universal_wallet_ledger(tenant_id, customer_id);

ALTER TABLE universal_wallet_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_universal_wallet_ledger ON universal_wallet_ledger;
CREATE POLICY tenant_isolation_universal_wallet_ledger ON universal_wallet_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_universal_wallet_ledger ON universal_wallet_ledger;
END
$$;

DROP TABLE IF EXISTS universal_wallet_ledger CASCADE;
