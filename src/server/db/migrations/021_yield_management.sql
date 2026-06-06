-- Autonomous Yield Management Engine
-- Issue #24192

CREATE TABLE IF NOT EXISTS yield_rules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    trigger_threshold_percent INTEGER NOT NULL, -- e.g., 50% empty
    discount_percent INTEGER NOT NULL, -- e.g., 15% discount
    target_audience TEXT NOT NULL, -- e.g., "past_customers", "waitlist"
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_yield_rules_tenant
ON yield_rules(tenant_id, status);

ALTER TABLE yield_rules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_yield_rules ON yield_rules;
CREATE POLICY tenant_isolation_yield_rules
ON yield_rules
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS yield_opportunities (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    target_date DATE NOT NULL,
    empty_slots INTEGER NOT NULL,
    total_slots INTEGER NOT NULL,
    utilization_percent INTEGER NOT NULL,
    recommended_discount_percent INTEGER NOT NULL,
    target_audience TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING_APPROVAL', -- PENDING_APPROVAL, APPROVED, DISMISSED
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_yield_opportunities_tenant
ON yield_opportunities(tenant_id, status);

ALTER TABLE yield_opportunities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_yield_opportunities ON yield_opportunities;
CREATE POLICY tenant_isolation_yield_opportunities
ON yield_opportunities
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
