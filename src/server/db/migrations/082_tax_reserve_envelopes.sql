-- Virtual Envelopes / Reserves schema for the CFO Agent

CREATE TABLE IF NOT EXISTS ledger_reserves (
    tenant_id TEXT NOT NULL,
    envelope_id TEXT NOT NULL,
    envelope_type TEXT NOT NULL, -- 'tax', 'liability', 'general'
    balance DOUBLE PRECISION DEFAULT 0.0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, envelope_id)
);

CREATE INDEX IF NOT EXISTS idx_ledger_reserves_tenant ON ledger_reserves(tenant_id);

ALTER TABLE ledger_reserves ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_reserves ON ledger_reserves;
CREATE POLICY tenant_isolation_ledger_reserves ON ledger_reserves USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
