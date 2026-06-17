-- Migration 139: Automated Product Subscriptions & Replenishment
-- Formalizes the schema for subscription plans, subscribers, and fulfillment schedules.

-- 1. Subscription Plans
CREATE TABLE IF NOT EXISTS subscription_plans (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    price_cents BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    frequency TEXT NOT NULL, -- 'weekly', 'monthly', 'yearly'
    interval TEXT, -- alias for frequency
    discount_percentage INTEGER DEFAULT 0,
    cutoff_day INTEGER,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(product_id, tenant_id)
);

-- 2. Subscribers
CREATE TABLE IF NOT EXISTS subscribers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    subscription_plan_id TEXT NOT NULL REFERENCES subscription_plans(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'ACTIVE', -- 'ACTIVE', 'PAUSED', 'CANCELED', 'PAST_DUE'
    stripe_subscription_id TEXT,
    current_period_end BIGINT,
    predicted_restock_date BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. Fulfillment Schedules
CREATE TABLE IF NOT EXISTS fulfillment_schedules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subscription_plan_id TEXT NOT NULL REFERENCES subscription_plans(id) ON DELETE CASCADE,
    fulfillment_date DATE NOT NULL,
    subscriber_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'PENDING', -- 'PENDING', 'PROCESSING', 'COMPLETED', 'FAILED'
    label_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- RLS Policies
ALTER TABLE subscription_plans ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_subscription_plans ON subscription_plans
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE subscribers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_subscribers ON subscribers
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE fulfillment_schedules ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_fulfillment_schedules ON fulfillment_schedules
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Indexes
CREATE INDEX IF NOT EXISTS idx_subscription_plans_tenant ON subscription_plans(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscription_plans_product ON subscription_plans(product_id);
CREATE INDEX IF NOT EXISTS idx_subscribers_tenant ON subscribers(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscribers_customer ON subscribers(customer_id);
CREATE INDEX IF NOT EXISTS idx_subscribers_plan ON subscribers(subscription_plan_id);
CREATE INDEX IF NOT EXISTS idx_fulfillment_schedules_tenant ON fulfillment_schedules(tenant_id);
CREATE INDEX IF NOT EXISTS idx_fulfillment_schedules_plan ON fulfillment_schedules(subscription_plan_id);
CREATE INDEX IF NOT EXISTS idx_fulfillment_schedules_date ON fulfillment_schedules(fulfillment_date);
