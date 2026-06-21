CREATE TABLE IF NOT EXISTS loyalty_programs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    program_type TEXT NOT NULL, -- 'points', 'punch_card', 'tiers'
    config JSONB DEFAULT '{}', -- specific rules, points per dollar, punch goal, etc.
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_loyalty_programs_tenant ON loyalty_programs(tenant_id);

CREATE TABLE IF NOT EXISTS customer_loyalty_accounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    program_id TEXT NOT NULL REFERENCES loyalty_programs(id),
    customer_id TEXT NOT NULL,
    points_balance INTEGER DEFAULT 0,
    punches INTEGER DEFAULT 0,
    current_tier TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, program_id, customer_id)
);

CREATE INDEX IF NOT EXISTS idx_customer_loyalty_accounts_tenant_customer ON customer_loyalty_accounts(tenant_id, customer_id);

CREATE TABLE IF NOT EXISTS loyalty_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    account_id TEXT NOT NULL REFERENCES customer_loyalty_accounts(id),
    transaction_type TEXT NOT NULL, -- 'earn', 'redeem', 'adjust'
    amount INTEGER NOT NULL, -- points or punches
    reason TEXT,
    order_id TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_loyalty_transactions_account ON loyalty_transactions(account_id);

CREATE TABLE IF NOT EXISTS rewards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    program_id TEXT NOT NULL REFERENCES loyalty_programs(id),
    name TEXT NOT NULL,
    description TEXT,
    cost_in_points INTEGER,
    cost_in_punches INTEGER,
    reward_type TEXT NOT NULL, -- 'discount', 'free_item', 'tier_upgrade'
    config JSONB DEFAULT '{}', -- specifics of the reward (e.g., discount percentage)
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_rewards_program ON rewards(program_id);

-- Enable RLS
ALTER TABLE loyalty_programs ENABLE ROW LEVEL SECURITY;
ALTER TABLE customer_loyalty_accounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE loyalty_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE rewards ENABLE ROW LEVEL SECURITY;

-- Create RLS Policies
CREATE POLICY tenant_isolation_loyalty_programs
ON loyalty_programs
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_customer_loyalty_accounts
ON customer_loyalty_accounts
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_loyalty_transactions
ON loyalty_transactions
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_rewards
ON rewards
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
