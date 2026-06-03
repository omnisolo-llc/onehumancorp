CREATE TABLE IF NOT EXISTS tax_jurisdictions (
    id TEXT PRIMARY KEY,
    country_code TEXT NOT NULL,
    state_code TEXT,
    zip_code TEXT,
    base_rate FLOAT NOT NULL DEFAULT 0.0,
    rules JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tax_jurisdictions_location ON tax_jurisdictions(country_code, state_code, zip_code);


CREATE TABLE IF NOT EXISTS tax_ledgers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    jurisdiction_id TEXT NOT NULL,
    taxable_amount FLOAT NOT NULL,
    tax_amount FLOAT NOT NULL,
    product_category TEXT,
    collected_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tax_ledgers_tenant ON tax_ledgers(tenant_id, collected_at DESC);
CREATE INDEX IF NOT EXISTS idx_tax_ledgers_jurisdiction ON tax_ledgers(jurisdiction_id);

ALTER TABLE tax_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tax_ledgers ON tax_ledgers;
CREATE POLICY tenant_isolation_tax_ledgers
ON tax_ledgers
USING (tenant_id = current_setting('app.current_tenant', true));
