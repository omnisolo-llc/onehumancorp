-- +goose Up
-- Migration 145: Multi-Tenant Loyalty and Rewards Engine Core

-- Define Loyalty Programs (e.g. Points, Punch Cards, Tiers)
CREATE TABLE IF NOT EXISTS loyalty_programs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    program_type TEXT NOT NULL, -- 'points', 'punch_card', 'tiers'
    config JSONB NOT NULL DEFAULT '{}', -- E.g. {"points_per_dollar": 1, "currency": "USD"}
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_loyalty_programs_tenant ON loyalty_programs(tenant_id);

ALTER TABLE IF EXISTS loyalty_programs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_programs ON loyalty_programs;
CREATE POLICY tenant_isolation_loyalty_programs ON loyalty_programs
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Customer Loyalty Accounts (Progress per program)
CREATE TABLE IF NOT EXISTS customer_loyalty_accounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    program_id TEXT NOT NULL REFERENCES loyalty_programs(id),
    points_balance INTEGER DEFAULT 0,
    punches_count INTEGER DEFAULT 0,
    tier_name TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, customer_id, program_id)
);
CREATE INDEX IF NOT EXISTS idx_cust_loyalty_accounts_tenant_cust ON customer_loyalty_accounts(tenant_id, customer_id);

ALTER TABLE IF EXISTS customer_loyalty_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts;
CREATE POLICY tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Loyalty Transactions (Audit log of points earned/redeemed)
CREATE TABLE IF NOT EXISTS loyalty_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    account_id TEXT NOT NULL REFERENCES customer_loyalty_accounts(id),
    transaction_type TEXT NOT NULL, -- 'EARN', 'REDEEM', 'ADJUST'
    points INTEGER NOT NULL, -- can be negative for redemption/adjustment
    reason TEXT,
    order_id TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_loyalty_transactions_account ON loyalty_transactions(tenant_id, account_id);

ALTER TABLE IF EXISTS loyalty_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_transactions ON loyalty_transactions;
CREATE POLICY tenant_isolation_loyalty_transactions ON loyalty_transactions
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Rewards (Redeemable items)
CREATE TABLE IF NOT EXISTS rewards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    program_id TEXT NOT NULL REFERENCES loyalty_programs(id),
    name TEXT NOT NULL,
    description TEXT,
    cost_in_points INTEGER NOT NULL,
    reward_type TEXT NOT NULL, -- 'discount', 'free_item'
    reward_value JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_rewards_tenant_program ON rewards(tenant_id, program_id);

ALTER TABLE IF EXISTS rewards ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_rewards ON rewards;
CREATE POLICY tenant_isolation_rewards ON rewards
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- +goose Down
DROP TABLE IF EXISTS rewards;
DROP TABLE IF EXISTS loyalty_transactions;
DROP TABLE IF EXISTS customer_loyalty_accounts;
DROP TABLE IF EXISTS loyalty_programs;
