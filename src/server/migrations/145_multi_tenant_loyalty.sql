CREATE TABLE IF NOT EXISTS loyalty_programs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    program_type TEXT NOT NULL, -- 'POINTS', 'PUNCH_CARD', 'TIERS'
    name TEXT NOT NULL,
    config JSONB DEFAULT '{}'::jsonb,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_loyalty_programs_tenant ON loyalty_programs(tenant_id);

CREATE TABLE IF NOT EXISTS customer_loyalty_accounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    program_id TEXT NOT NULL,
    points_balance INTEGER DEFAULT 0,
    punch_count INTEGER DEFAULT 0,
    tier_name TEXT,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, customer_id, program_id)
);

CREATE INDEX IF NOT EXISTS idx_customer_loyalty_accounts_tenant_customer ON customer_loyalty_accounts(tenant_id, customer_id);

CREATE TABLE IF NOT EXISTS loyalty_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL, -- 'EARN', 'REDEEM'
    points INTEGER NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_loyalty_transactions_account ON loyalty_transactions(tenant_id, account_id);

CREATE TABLE IF NOT EXISTS rewards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    program_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    points_required INTEGER NOT NULL,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_rewards_program ON rewards(tenant_id, program_id);

ALTER TABLE loyalty_programs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_programs ON loyalty_programs;
CREATE POLICY tenant_isolation_loyalty_programs
ON loyalty_programs
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE customer_loyalty_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts;
CREATE POLICY tenant_isolation_customer_loyalty_accounts
ON customer_loyalty_accounts
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE loyalty_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_transactions ON loyalty_transactions;
CREATE POLICY tenant_isolation_loyalty_transactions
ON loyalty_transactions
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE rewards ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_rewards ON rewards;
CREATE POLICY tenant_isolation_rewards
ON rewards
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
