-- Create tax obligations table for Universal Embedded Finance & AI Taxation Ledger
CREATE TABLE IF NOT EXISTS tax_obligations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    ledger_entry_id TEXT NOT NULL,
    tax_type TEXT NOT NULL,
    amount_estimated DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tax_obligations_tenant ON tax_obligations(tenant_id);
CREATE INDEX IF NOT EXISTS idx_tax_obligations_ledger_entry ON tax_obligations(ledger_entry_id);

ALTER TABLE tax_obligations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tax_obligations ON tax_obligations;
CREATE POLICY tenant_isolation_tax_obligations ON tax_obligations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Update ledger queries inside sqlite tests
