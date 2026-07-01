-- +goose Up
-- Migration: Autonomous Loyalty & VIP Membership Engine

ALTER TABLE IF EXISTS customers ADD COLUMN IF NOT EXISTS loyalty_tier TEXT;
ALTER TABLE IF EXISTS customers ADD COLUMN IF NOT EXISTS lifetime_value_cents BIGINT DEFAULT 0;

CREATE TABLE IF NOT EXISTS loyalty_tier_configs (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    tier_name TEXT NOT NULL,
    min_spend_cents BIGINT NOT NULL,
    benefits JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE loyalty_tier_configs ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_loyalty_tier_configs ON loyalty_tier_configs;
CREATE POLICY tenant_isolation_loyalty_tier_configs ON loyalty_tier_configs
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_loyalty_tier_configs ON loyalty_tier_configs;
DROP TABLE IF EXISTS loyalty_tier_configs CASCADE;
ALTER TABLE IF EXISTS customers DROP COLUMN IF EXISTS lifetime_value_cents;
ALTER TABLE IF EXISTS customers DROP COLUMN IF EXISTS loyalty_tier;
