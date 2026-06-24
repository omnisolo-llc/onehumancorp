-- +goose Up
CREATE TABLE IF NOT EXISTS universal_tax_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    transaction_id TEXT,
    invoice_id TEXT,
    currency TEXT NOT NULL DEFAULT 'USD',
    base_currency TEXT NOT NULL DEFAULT 'USD',
    exchange_rate DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    total_amount DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    tax_amount DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    tax_type TEXT NOT NULL,
    direction TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_universal_tax_ledger_tenant ON universal_tax_ledger(tenant_id);
CREATE INDEX IF NOT EXISTS idx_universal_tax_ledger_invoice ON universal_tax_ledger(invoice_id);
ALTER TABLE universal_tax_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_universal_tax_ledger ON universal_tax_ledger;
CREATE POLICY tenant_isolation_universal_tax_ledger ON universal_tax_ledger
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS exchange_rate DOUBLE PRECISION DEFAULT 1.0;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS tax_amount DOUBLE PRECISION DEFAULT 0.0;
-- +goose Down
DROP TABLE IF EXISTS universal_tax_ledger;
