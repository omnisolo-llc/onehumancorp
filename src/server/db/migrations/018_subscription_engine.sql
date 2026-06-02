CREATE TABLE IF NOT EXISTS subscription_plans (
    id UUID PRIMARY KEY,
    organization_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    price_cents BIGINT NOT NULL,
    billing_interval TEXT NOT NULL DEFAULT 'month',
    stripe_product_id TEXT,
    stripe_price_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_subscription_plans_org ON subscription_plans(organization_id);

ALTER TABLE subscription_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
CREATE POLICY tenant_isolation_subscription_plans ON subscription_plans USING (organization_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS subscribers (
    id UUID PRIMARY KEY,
    organization_id TEXT NOT NULL,
    subscription_plan_id UUID REFERENCES subscription_plans(id) ON DELETE CASCADE,
    customer_name TEXT NOT NULL,
    customer_email TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    stripe_subscription_id TEXT,
    stripe_customer_id TEXT,
    current_period_end TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_subscribers_org ON subscribers(organization_id);
CREATE INDEX IF NOT EXISTS idx_subscribers_plan ON subscribers(subscription_plan_id);
CREATE INDEX IF NOT EXISTS idx_subscribers_status ON subscribers(status);

ALTER TABLE subscribers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
CREATE POLICY tenant_isolation_subscribers ON subscribers USING (organization_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS fulfillment_batches (
    id UUID PRIMARY KEY,
    organization_id TEXT NOT NULL,
    subscription_plan_id UUID REFERENCES subscription_plans(id) ON DELETE CASCADE,
    fulfillment_date DATE NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    total_boxes INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_fulfillment_batches_org ON fulfillment_batches(organization_id);
CREATE INDEX IF NOT EXISTS idx_fulfillment_batches_plan_date ON fulfillment_batches(subscription_plan_id, fulfillment_date);

ALTER TABLE fulfillment_batches ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_fulfillment_batches ON fulfillment_batches;
CREATE POLICY tenant_isolation_fulfillment_batches ON fulfillment_batches USING (organization_id::text = current_setting('app.current_tenant', true));
