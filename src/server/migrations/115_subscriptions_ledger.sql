-- Migration to add subscriptions and subscription_events tables

CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    stripe_subscription_id TEXT,
    status TEXT NOT NULL DEFAULT 'active', -- active, past_due, canceled, paused
    current_period_end TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS subscription_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subscription_id TEXT NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL, -- created, renewed, failed, paused
    agent_id TEXT, -- nullable, references agents if AI triggered
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscription_events ENABLE ROW LEVEL SECURITY;

-- Create policies for subscriptions
CREATE POLICY tenant_isolation_subscriptions_select ON subscriptions
    FOR SELECT USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_subscriptions_insert ON subscriptions
    FOR INSERT WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_subscriptions_update ON subscriptions
    FOR UPDATE USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_subscriptions_delete ON subscriptions
    FOR DELETE USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Create policies for subscription_events
CREATE POLICY tenant_isolation_subscription_events_select ON subscription_events
    FOR SELECT USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_subscription_events_insert ON subscription_events
    FOR INSERT WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_subscription_events_update ON subscription_events
    FOR UPDATE USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_subscription_events_delete ON subscription_events
    FOR DELETE USING (tenant_id::text = current_setting('app.current_tenant', true));
