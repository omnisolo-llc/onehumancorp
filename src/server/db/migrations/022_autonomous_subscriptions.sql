CREATE TABLE IF NOT EXISTS tenant_subscriptions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active', -- 'active', 'past_due', 'canceled'
    balance INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tenant_subscriptions_tenant_id ON tenant_subscriptions(tenant_id);

ALTER TABLE tenant_subscriptions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tenant_subscriptions ON tenant_subscriptions;
CREATE POLICY tenant_isolation_tenant_subscriptions
ON tenant_subscriptions
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS tenant_subscription_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    subscription_id TEXT NOT NULL REFERENCES tenant_subscriptions(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL, -- 'redemption', 'renewal'
    delta INTEGER NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tenant_subscription_events_tenant_id ON tenant_subscription_events(tenant_id);

ALTER TABLE tenant_subscription_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tenant_subscription_events ON tenant_subscription_events;
CREATE POLICY tenant_isolation_tenant_subscription_events
ON tenant_subscription_events
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
