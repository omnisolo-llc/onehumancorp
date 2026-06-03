-- Autonomous AI Tax and Compliance Engine
-- GitHub Issue #23269

CREATE TABLE IF NOT EXISTS tax_jurisdictions (
    id TEXT PRIMARY KEY,
    country_code TEXT NOT NULL,
    region_code TEXT,
    tax_rate DECIMAL(5,4) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tax_jurisdictions_location ON tax_jurisdictions(country_code, region_code);


CREATE TABLE IF NOT EXISTS tax_ledgers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    jurisdiction_id TEXT NOT NULL,
    taxable_amount_cents BIGINT NOT NULL,
    tax_rate DECIMAL(5,4) NOT NULL,
    tax_collected_cents BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tax_ledgers_tenant_date ON tax_ledgers(tenant_id, created_at);

ALTER TABLE tax_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tax_ledgers ON tax_ledgers;
CREATE POLICY tenant_isolation_tax_ledgers ON tax_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Implement append-only constraint via trigger for tax_ledgers
CREATE OR REPLACE FUNCTION prevent_tax_ledger_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'tax_ledgers is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_tax_ledger_update ON tax_ledgers;
CREATE TRIGGER trg_append_only_tax_ledger_update
BEFORE UPDATE ON tax_ledgers
FOR EACH ROW EXECUTE FUNCTION prevent_tax_ledger_update_or_delete();

DROP TRIGGER IF EXISTS trg_append_only_tax_ledger_delete ON tax_ledgers;
CREATE TRIGGER trg_append_only_tax_ledger_delete
BEFORE DELETE ON tax_ledgers
FOR EACH ROW EXECUTE FUNCTION prevent_tax_ledger_update_or_delete();
