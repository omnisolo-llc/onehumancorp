-- +goose Up
CREATE TABLE IF NOT EXISTS service_leads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    description TEXT,
    images JSONB,
    source TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'new' CHECK (status IN ('new', 'estimating', 'estimated', 'booked', 'closed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_service_leads_tenant_id ON service_leads(tenant_id);

ALTER TABLE service_leads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_leads ON service_leads;
CREATE POLICY tenant_isolation_service_leads
ON service_leads
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS estimates (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_lead_id TEXT REFERENCES service_leads(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE CASCADE,
    description TEXT,
    min_price_cents BIGINT,
    max_price_cents BIGINT,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'sent', 'approved', 'rejected', 'expired')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_estimates_tenant_id ON estimates(tenant_id);

ALTER TABLE estimates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_estimates ON estimates;
CREATE POLICY tenant_isolation_estimates
ON estimates
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS deposit_requirements (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    estimate_id TEXT NOT NULL REFERENCES estimates(id) ON DELETE CASCADE,
    amount_cents BIGINT NOT NULL,
    percentage DECIMAL(5,2),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'paid', 'refunded', 'voided')),
    payment_intent_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_deposit_requirements_tenant_id ON deposit_requirements(tenant_id);

ALTER TABLE deposit_requirements ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_deposit_requirements ON deposit_requirements;
CREATE POLICY tenant_isolation_deposit_requirements
ON deposit_requirements
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_deposit_requirements ON deposit_requirements;
DROP TABLE IF EXISTS deposit_requirements CASCADE;

DROP POLICY IF EXISTS tenant_isolation_estimates ON estimates;
DROP TABLE IF EXISTS estimates CASCADE;

DROP POLICY IF EXISTS tenant_isolation_service_leads ON service_leads;
DROP TABLE IF EXISTS service_leads CASCADE;
