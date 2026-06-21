-- +goose Up
-- Loyalty Programs
CREATE TABLE IF NOT EXISTS loyalty_programs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    program_type TEXT NOT NULL, -- 'POINTS', 'PUNCH_CARD', 'TIERS'
    config JSONB NOT NULL DEFAULT '{}'::jsonb,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_loyalty_programs_tenant ON loyalty_programs(tenant_id);
ALTER TABLE loyalty_programs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_programs ON loyalty_programs;
CREATE POLICY tenant_isolation_loyalty_programs ON loyalty_programs USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Customer Loyalty Accounts
CREATE TABLE IF NOT EXISTS customer_loyalty_accounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    program_id TEXT NOT NULL,
    points_balance INTEGER NOT NULL DEFAULT 0,
    tier TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, customer_id, program_id)
);
CREATE INDEX IF NOT EXISTS idx_customer_loyalty_accounts_tenant ON customer_loyalty_accounts(tenant_id);
ALTER TABLE customer_loyalty_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts;
CREATE POLICY tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Loyalty Transactions
CREATE TABLE IF NOT EXISTS loyalty_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL, -- 'EARN', 'REDEEM', 'EXPIRE'
    points INTEGER NOT NULL,
    description TEXT,
    order_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_loyalty_transactions_tenant ON loyalty_transactions(tenant_id);
CREATE INDEX IF NOT EXISTS idx_loyalty_transactions_account ON loyalty_transactions(account_id);
ALTER TABLE loyalty_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_transactions ON loyalty_transactions;
CREATE POLICY tenant_isolation_loyalty_transactions ON loyalty_transactions USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Rewards
CREATE TABLE IF NOT EXISTS rewards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    program_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    points_cost INTEGER NOT NULL,
    reward_type TEXT NOT NULL, -- 'DISCOUNT', 'FREE_ITEM'
    config JSONB NOT NULL DEFAULT '{}'::jsonb,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_rewards_tenant ON rewards(tenant_id);
CREATE INDEX IF NOT EXISTS idx_rewards_program ON rewards(program_id);
ALTER TABLE rewards ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_rewards ON rewards;
CREATE POLICY tenant_isolation_rewards ON rewards USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS rewards;
DROP TABLE IF EXISTS loyalty_transactions;
DROP TABLE IF EXISTS customer_loyalty_accounts;
DROP TABLE IF EXISTS loyalty_programs;
