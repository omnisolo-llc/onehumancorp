-- +goose Up
CREATE TABLE IF NOT EXISTS subscription_health_scores (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subscription_id TEXT NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    score INTEGER NOT NULL, -- 0 to 100
    risk_level TEXT NOT NULL, -- 'LOW', 'MEDIUM', 'HIGH'
    factors JSONB NOT NULL DEFAULT '[]', -- Array of reasons for the score
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_subscription_health_scores_tenant_id ON subscription_health_scores(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscription_health_scores_subscription_id ON subscription_health_scores(subscription_id);
CREATE INDEX IF NOT EXISTS idx_subscription_health_scores_customer_id ON subscription_health_scores(customer_id);

ALTER TABLE subscription_health_scores ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscription_health_scores ON subscription_health_scores;
CREATE POLICY tenant_isolation_subscription_health_scores
ON subscription_health_scores
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS subscription_retention_interventions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subscription_id TEXT NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    health_score_id TEXT REFERENCES subscription_health_scores(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'DRAFTED', -- 'DRAFTED', 'APPROVED', 'REJECTED', 'SENT', 'FAILED'
    intervention_type TEXT NOT NULL, -- 'WIN_BACK_OFFER', 'DUNNING', 'CHECK_IN'
    channel TEXT NOT NULL, -- 'EMAIL', 'SMS', 'INAPP'
    proposed_message TEXT NOT NULL,
    owner_feed_item_id TEXT REFERENCES tenant_feed_items(id) ON DELETE SET NULL,
    approved_at TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_subscription_retention_interventions_tenant_id ON subscription_retention_interventions(tenant_id);
CREATE INDEX IF NOT EXISTS idx_subscription_retention_interventions_subscription_id ON subscription_retention_interventions(subscription_id);

ALTER TABLE subscription_retention_interventions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscription_retention_interventions ON subscription_retention_interventions;
CREATE POLICY tenant_isolation_subscription_retention_interventions
ON subscription_retention_interventions
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS subscription_retention_interventions CASCADE;
DROP TABLE IF EXISTS subscription_health_scores CASCADE;
