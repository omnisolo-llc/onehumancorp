-- +goose Up
-- Migration 153: AI-Driven Omnichannel Loyalty & Referral Loop Architecture

-- loyalty_events table
CREATE TABLE IF NOT EXISTS loyalty_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL, -- e.g., 'purchase', 'referral', 'positive_feedback'
    event_points INTEGER NOT NULL DEFAULT 0,
    source TEXT, -- 'pos', 'ecommerce', etc.
    reference_id TEXT, -- e.g. transaction ID, referral link ID
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE loyalty_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_loyalty_events ON loyalty_events USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE INDEX idx_loyalty_events_tenant_customer ON loyalty_events(tenant_id, customer_id);

-- referral_links table
CREATE TABLE IF NOT EXISTS referral_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    link_code TEXT NOT NULL UNIQUE,
    campaign_name TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE referral_links ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_referral_links ON referral_links USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE INDEX idx_referral_links_tenant_code ON referral_links(tenant_id, link_code);
CREATE INDEX idx_referral_links_tenant_customer ON referral_links(tenant_id, customer_id);

-- loyalty_settings table (zero-configuration defaults)
CREATE TABLE IF NOT EXISTS loyalty_settings (
    tenant_id TEXT PRIMARY KEY,
    is_enabled BOOLEAN DEFAULT FALSE,
    reward_threshold_points INTEGER DEFAULT 100,
    reward_type TEXT DEFAULT 'discount', -- 'discount', 'free_shipping', etc.
    reward_value NUMERIC(10, 2) DEFAULT 10.00,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE loyalty_settings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_loyalty_settings ON loyalty_settings USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


ALTER TABLE customers ADD COLUMN referral_code TEXT;


-- +goose Down
ALTER TABLE customers DROP COLUMN referral_code;

DROP POLICY IF EXISTS tenant_isolation_loyalty_settings ON loyalty_settings;
DROP TABLE IF EXISTS loyalty_settings CASCADE;

DROP POLICY IF EXISTS tenant_isolation_referral_links ON referral_links;
DROP TABLE IF EXISTS referral_links CASCADE;

DROP POLICY IF EXISTS tenant_isolation_loyalty_events ON loyalty_events;
DROP TABLE IF EXISTS loyalty_events CASCADE;
