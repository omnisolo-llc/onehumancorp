-- Migration to support CRDT-based Offline-First Subscriptions

CREATE TABLE IF NOT EXISTS entitlements (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subscription_id TEXT NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    credit_balance INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_entitlements_tenant_id ON entitlements(tenant_id);
CREATE INDEX IF NOT EXISTS idx_entitlements_subscription_id ON entitlements(subscription_id);

ALTER TABLE entitlements ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_entitlements ON entitlements;
CREATE POLICY tenant_isolation_entitlements
ON entitlements
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS subscription_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subscription_id TEXT NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL, -- 'credit_redeemed', 'payment_failed', 'status_changed'
    event_payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    clock INTEGER NOT NULL DEFAULT 0,
    signature TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_subscription_events_tenant_id ON subscription_events(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscription_events_subscription_id ON subscription_events(subscription_id);

ALTER TABLE subscription_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscription_events ON subscription_events;
CREATE POLICY tenant_isolation_subscription_events
ON subscription_events
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Implement append-only constraint via trigger for subscription_events
CREATE OR REPLACE FUNCTION prevent_subscription_events_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'subscription_events is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_subscription_events_update ON subscription_events;
CREATE TRIGGER trg_append_only_subscription_events_update
BEFORE UPDATE ON subscription_events
FOR EACH ROW EXECUTE FUNCTION prevent_subscription_events_update_or_delete();

DROP TRIGGER IF EXISTS trg_append_only_subscription_events_delete ON subscription_events;
CREATE TRIGGER trg_append_only_subscription_events_delete
BEFORE DELETE ON subscription_events
FOR EACH ROW EXECUTE FUNCTION prevent_subscription_events_update_or_delete();
