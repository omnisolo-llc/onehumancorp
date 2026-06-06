CREATE TABLE IF NOT EXISTS lead_gen_campaigns (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    budget NUMERIC NOT NULL,
    radius_miles NUMERIC NOT NULL,
    zip_code TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_lead_gen_campaigns_tenant ON lead_gen_campaigns(tenant_id, created_at DESC);

ALTER TABLE lead_gen_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns;
CREATE POLICY tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
