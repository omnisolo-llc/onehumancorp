CREATE TABLE IF NOT EXISTS lead_gen_campaigns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    budget NUMERIC(10, 2) NOT NULL,
    radius_miles INT NOT NULL,
    zip_code VARCHAR(20) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_lead_gen_campaign_tenant FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
);

ALTER TABLE lead_gen_campaigns ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
