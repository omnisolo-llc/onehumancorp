CREATE TABLE IF NOT EXISTS subscription_plans (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    price_cents INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    frequency TEXT NOT NULL, -- e.g. 'monthly', 'weekly'
    cutoff_day INTEGER, -- e.g. 5 for the 5th of the month
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_subscription_plans_tenant ON subscription_plans(tenant_id);
ALTER TABLE subscription_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
CREATE POLICY tenant_isolation_subscription_plans
ON subscription_plans
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE TABLE IF NOT EXISTS subscribers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    subscription_plan_id TEXT NOT NULL REFERENCES subscription_plans(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'ACTIVE', -- ACTIVE, PAST_DUE, CANCELED
    stripe_subscription_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_subscribers_tenant ON subscribers(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscribers_plan ON subscribers(subscription_plan_id);
ALTER TABLE subscribers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
CREATE POLICY tenant_isolation_subscribers
ON subscribers
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE TABLE IF NOT EXISTS fulfillment_batches (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    subscription_plan_id TEXT NOT NULL REFERENCES subscription_plans(id) ON DELETE CASCADE,
    fulfillment_date DATE NOT NULL,
    subscriber_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'PENDING', -- PENDING, LABELS_PRINTED, FULFILLED
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_fulfillment_batches_tenant ON fulfillment_batches(tenant_id);
CREATE INDEX IF NOT EXISTS idx_fulfillment_batches_plan ON fulfillment_batches(subscription_plan_id);
ALTER TABLE fulfillment_batches ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_fulfillment_batches ON fulfillment_batches;
CREATE POLICY tenant_isolation_fulfillment_batches
ON fulfillment_batches
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));