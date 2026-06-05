-- +goose Up
-- Migration 076: Add subscription tables

CREATE TABLE IF NOT EXISTS subscription_plans (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id TEXT,
    name TEXT,
    description TEXT,
    amount BIGINT,
    currency TEXT DEFAULT 'USD',
    interval TEXT NOT NULL,
    active BOOLEAN DEFAULT true,
    discount_percentage INTEGER DEFAULT 0,
    created_at BIGINT
);

CREATE TABLE IF NOT EXISTS subscribers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    plan_id TEXT,
    customer_id TEXT NOT NULL,
    stripe_subscription_id TEXT,
    status TEXT NOT NULL,
    current_period_end BIGINT,
    created_at BIGINT
);

CREATE TABLE IF NOT EXISTS fulfillment_batches (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    plan_id TEXT,
    target_date BIGINT,
    status TEXT,
    created_at BIGINT
);

CREATE INDEX IF NOT EXISTS idx_subscription_plans_tenant ON subscription_plans(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscribers_tenant ON subscribers(tenant_id);

ALTER TABLE subscription_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
CREATE POLICY tenant_isolation_subscription_plans
ON subscription_plans
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE subscribers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
CREATE POLICY tenant_isolation_subscribers
ON subscribers
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS fulfillment_batches CASCADE;
DROP TABLE IF EXISTS subscribers CASCADE;
DROP TABLE IF EXISTS subscription_plans CASCADE;
