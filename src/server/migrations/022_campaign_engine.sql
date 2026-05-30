-- Migration 022: Campaign Engine Schema
CREATE TABLE IF NOT EXISTS campaigns (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    goal TEXT NOT NULL,
    status TEXT NOT NULL, -- Draft, Active, Paused, Completed
    start_time TIMESTAMPTZ,
    end_time TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE campaigns ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_campaigns ON campaigns USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS campaign_assets (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    campaign_id TEXT REFERENCES campaigns(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    content_url TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE campaign_assets ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_campaign_assets ON campaign_assets USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS channel_executions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    campaign_id TEXT REFERENCES campaigns(id) ON DELETE CASCADE,
    channel TEXT NOT NULL,
    metrics_sent INTEGER DEFAULT 0,
    metrics_clicks INTEGER DEFAULT 0,
    metrics_conversions INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE channel_executions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_channel_executions ON channel_executions USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS promotion_codes (
    code TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    campaign_id TEXT REFERENCES campaigns(id) ON DELETE CASCADE,
    discount_value DOUBLE PRECISION NOT NULL,
    discount_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE promotion_codes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_promotion_codes ON promotion_codes USING (tenant_id::text = current_setting('app.current_tenant', true));
