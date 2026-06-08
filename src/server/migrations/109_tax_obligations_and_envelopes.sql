-- Create tax_obligations table
CREATE TABLE IF NOT EXISTS tax_obligations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    tax_amount DOUBLE PRECISION NOT NULL,
    tax_jurisdiction TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING', -- PENDING, REMITTED
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tax_obligations_tenant ON tax_obligations(tenant_id);

ALTER TABLE tax_obligations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tax_obligations ON tax_obligations;
CREATE POLICY tenant_isolation_tax_obligations ON tax_obligations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- Create virtual_envelopes table
CREATE TABLE IF NOT EXISTS virtual_envelopes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    purpose TEXT NOT NULL,
    balance DOUBLE PRECISION DEFAULT 0.0,
    allocation_percentage DOUBLE PRECISION DEFAULT 0.0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_virtual_envelopes_tenant ON virtual_envelopes(tenant_id);

ALTER TABLE virtual_envelopes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_virtual_envelopes ON virtual_envelopes;
CREATE POLICY tenant_isolation_virtual_envelopes ON virtual_envelopes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
