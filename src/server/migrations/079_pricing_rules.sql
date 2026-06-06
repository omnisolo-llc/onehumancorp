-- +goose Up
-- Migration 079: Multi-Currency Pricing Rules & Base Currency

ALTER TABLE tenants ADD COLUMN IF NOT EXISTS tenant_base_currency TEXT DEFAULT 'USD';
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS fx_variance_bucket BIGINT DEFAULT 0;

CREATE TABLE IF NOT EXISTS ohc_localized_pricing_rules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    rule_type TEXT NOT NULL,
    charm_point TEXT NOT NULL,
    target_currency TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, target_currency)
);

ALTER TABLE ohc_localized_pricing_rules ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'ohc_localized_pricing_rules'
            AND policyname = 'tenant_isolation_ohc_localized_pricing_rules'
    ) THEN
        CREATE POLICY tenant_isolation_ohc_localized_pricing_rules
        ON ohc_localized_pricing_rules
        USING (tenant_id::text = current_setting('app.current_tenant', true))
        WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_ohc_localized_pricing_rules ON ohc_localized_pricing_rules;
END
$$;

DROP TABLE IF EXISTS ohc_localized_pricing_rules;
ALTER TABLE tenants DROP COLUMN IF NOT EXISTS fx_variance_bucket;
ALTER TABLE tenants DROP COLUMN IF NOT EXISTS tenant_base_currency;
