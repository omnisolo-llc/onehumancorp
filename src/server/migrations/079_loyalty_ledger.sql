DROP TABLE IF EXISTS loyalty_ledger CASCADE;
CREATE TABLE IF NOT EXISTS loyalty_ledgers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    points_balance INT NOT NULL DEFAULT 0,
    lifetime_points_earned INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS loyalty_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    order_id TEXT,
    points_change INT NOT NULL,
    transaction_type TEXT NOT NULL, -- 'earn', 'redeem', 'expire', 'refund'
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS referral_links (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL, -- The referrer
    referral_code TEXT UNIQUE NOT NULL,
    url TEXT NOT NULL,
    clicks INT NOT NULL DEFAULT 0,
    successful_referrals INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS loyalty_settings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT UNIQUE NOT NULL,
    is_enabled BOOLEAN NOT NULL DEFAULT false,
    reward_style TEXT NOT NULL DEFAULT 'points', -- 'points', 'cashback', 'surprise'
    points_per_currency_unit INT NOT NULL DEFAULT 1,
    currency_value_per_point DECIMAL(10, 4) NOT NULL DEFAULT 0.01,
    minimum_redemption_points INT NOT NULL DEFAULT 100,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
);

-- Row Level Security
ALTER TABLE loyalty_ledgers ENABLE ROW LEVEL SECURITY;
ALTER TABLE loyalty_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE referral_links ENABLE ROW LEVEL SECURITY;
ALTER TABLE loyalty_settings ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_loyalty_ledgers ON loyalty_ledgers
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_loyalty_transactions ON loyalty_transactions
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_referral_links ON referral_links
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_loyalty_settings ON loyalty_settings
    USING (tenant_id = current_setting('app.current_tenant', true));
