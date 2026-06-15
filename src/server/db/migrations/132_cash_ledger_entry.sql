-- +goose Up
CREATE TABLE IF NOT EXISTS cash_ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    terminal_session_id TEXT NOT NULL,
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    transaction_type TEXT NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_cash_ledger_entries_tenant ON cash_ledger_entries(tenant_id);
CREATE INDEX IF NOT EXISTS idx_cash_ledger_entries_session ON cash_ledger_entries(terminal_session_id);
ALTER TABLE cash_ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_cash_ledger_entries ON cash_ledger_entries;
CREATE POLICY tenant_isolation_cash_ledger_entries ON cash_ledger_entries USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS cash_ledger_entries;
