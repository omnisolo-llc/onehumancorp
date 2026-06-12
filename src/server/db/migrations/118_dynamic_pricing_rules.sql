-- Migration: Dynamic Pricing Rules
CREATE TABLE IF NOT EXISTS pricing_rules (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    base_price_cents BIGINT NOT NULL,
    rules_json JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE pricing_rules ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_pricing_rules ON pricing_rules USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
