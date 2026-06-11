-- Migration: Churn Prediction & Proactive Retention Engine
-- Add EngagementEvents, ChurnPredictions, and RetentionActions

CREATE TABLE IF NOT EXISTS engagement_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    customer_id VARCHAR NOT NULL,
    event_type VARCHAR NOT NULL, -- 'Booking', 'Purchase', 'Message'
    occurred_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS churn_predictions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    customer_id VARCHAR NOT NULL,
    probability DOUBLE PRECISION NOT NULL,
    primary_factor VARCHAR,
    predicted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS retention_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    prediction_id UUID NOT NULL REFERENCES churn_predictions(id) ON DELETE CASCADE,
    status VARCHAR NOT NULL DEFAULT 'Draft', -- 'Draft', 'Approved', 'Sent'
    proposed_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE engagement_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE churn_predictions ENABLE ROW LEVEL SECURITY;
ALTER TABLE retention_actions ENABLE ROW LEVEL SECURITY;

CREATE INDEX IF NOT EXISTS idx_engagement_events_tenant_customer ON engagement_events(tenant_id, customer_id);
CREATE INDEX IF NOT EXISTS idx_churn_predictions_tenant_customer ON churn_predictions(tenant_id, customer_id);
CREATE INDEX IF NOT EXISTS idx_retention_actions_tenant ON retention_actions(tenant_id);
DROP POLICY IF EXISTS tenant_isolation_engagement_events ON engagement_events;
CREATE POLICY tenant_isolation_engagement_events ON engagement_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_churn_predictions ON churn_predictions;
CREATE POLICY tenant_isolation_churn_predictions ON churn_predictions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_retention_actions ON retention_actions;
CREATE POLICY tenant_isolation_retention_actions ON retention_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE customer_identities ADD COLUMN IF NOT EXISTS risk_score DOUBLE PRECISION DEFAULT 0.0;
ALTER TABLE customer_identities ADD COLUMN IF NOT EXISTS last_engaged_at TIMESTAMP WITH TIME ZONE;
