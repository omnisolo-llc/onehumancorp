CREATE TABLE IF NOT EXISTS waitlist_campaigns (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('ACTIVE', 'PAUSED', 'CLOSED')),
    capacity_limit INTEGER,
    deposit_required BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS pre_order_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    waitlist_campaign_id TEXT REFERENCES waitlist_campaigns(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
    channel TEXT NOT NULL CHECK (channel IN ('WEB', 'INSTAGRAM', 'SMS', 'POS')),
    status TEXT NOT NULL CHECK (status IN ('PENDING', 'CONFIRMED', 'FULFILLED', 'CANCELLED')),
    deposit_amount DECIMAL NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE waitlist_campaigns ENABLE ROW LEVEL SECURITY;
CREATE POLICY waitlist_campaigns_tenant_isolation ON waitlist_campaigns
    USING (tenant_id = current_setting('app.current_tenant_id', true));

ALTER TABLE pre_order_entries ENABLE ROW LEVEL SECURITY;
CREATE POLICY pre_order_entries_tenant_isolation ON pre_order_entries
    USING (tenant_id = current_setting('app.current_tenant_id', true));
