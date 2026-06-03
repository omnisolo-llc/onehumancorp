-- Autonomous AI Tax and Compliance Engine Migration
-- Related to GitHub Issue #23269

CREATE TABLE IF NOT EXISTS ohc_tax_jurisdictions (
    id TEXT PRIMARY KEY,
    country_code TEXT NOT NULL,
    region_code TEXT,
    tax_rate NUMERIC NOT NULL,
    tax_type TEXT NOT NULL,
    effective_date TIMESTAMPTZ NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ohc_tax_jurisdictions_lookup
ON ohc_tax_jurisdictions(country_code, region_code);

CREATE TABLE IF NOT EXISTS ohc_tax_ledgers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    jurisdiction_id TEXT NOT NULL REFERENCES ohc_tax_jurisdictions(id),
    taxable_amount NUMERIC NOT NULL,
    tax_amount NUMERIC NOT NULL,
    tax_rate NUMERIC NOT NULL,
    status TEXT NOT NULL DEFAULT 'COLLECTED',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ohc_tax_ledgers_tenant
ON ohc_tax_ledgers(tenant_id, created_at DESC);

ALTER TABLE ohc_tax_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_tax_ledgers ON ohc_tax_ledgers;
CREATE POLICY tenant_isolation_ohc_tax_ledgers
ON ohc_tax_ledgers
USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS ohc_tax_nexus_thresholds (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    jurisdiction_id TEXT NOT NULL REFERENCES ohc_tax_jurisdictions(id),
    current_volume NUMERIC NOT NULL DEFAULT 0,
    threshold_volume NUMERIC NOT NULL,
    status TEXT NOT NULL DEFAULT 'MONITORING',
    last_checked_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_ohc_tax_nexus_thresholds_tenant
ON ohc_tax_nexus_thresholds(tenant_id);

ALTER TABLE ohc_tax_nexus_thresholds ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_tax_nexus_thresholds ON ohc_tax_nexus_thresholds;
CREATE POLICY tenant_isolation_ohc_tax_nexus_thresholds
ON ohc_tax_nexus_thresholds
USING (tenant_id = current_setting('app.current_tenant', true));
