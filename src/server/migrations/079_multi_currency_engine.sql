-- Offline-First Multi-Currency Engine Expansion

CREATE TABLE IF NOT EXISTS ohc_localized_pricing_rules (
    tenant_id TEXT PRIMARY KEY,
    tenant_base_currency TEXT NOT NULL DEFAULT 'USD',
    fx_variance_bucket BIGINT NOT NULL DEFAULT 0, -- in cents
    rounding_strategy TEXT NOT NULL DEFAULT 'nearest_99',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE ohc_localized_pricing_rules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_localized_pricing_rules ON ohc_localized_pricing_rules;
CREATE POLICY tenant_isolation_ohc_localized_pricing_rules
ON ohc_localized_pricing_rules
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
