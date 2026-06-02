-- Invisible Multi-Party Split Payments & Consignment Ledger

ALTER TABLE invoices ADD COLUMN IF NOT EXISTS split_config JSONB DEFAULT '{}'::jsonb;

CREATE TABLE IF NOT EXISTS invoice_splits (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    invoice_id TEXT NOT NULL,
    sub_merchant_id TEXT NOT NULL,
    amount_allocated double precision NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    transfer_job_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_invoice_splits_invoice ON invoice_splits(invoice_id);

ALTER TABLE invoice_splits ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoice_splits ON invoice_splits;
CREATE POLICY tenant_isolation_invoice_splits
ON invoice_splits
USING (tenant_id = current_setting('app.current_tenant', true));
