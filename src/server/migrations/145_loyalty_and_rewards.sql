-- Multi-Tenant Loyalty and Rewards Engine Core
-- GitHub Issue #29964

-- 1. loyalty_programs
CREATE TABLE IF NOT EXISTS loyalty_programs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    program_type TEXT NOT NULL, -- points, punch_card, tiers
    config JSONB NOT NULL DEFAULT '{}'::jsonb,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_loyalty_programs_tenant ON loyalty_programs(tenant_id);

ALTER TABLE loyalty_programs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_programs ON loyalty_programs;
CREATE POLICY tenant_isolation_loyalty_programs
ON loyalty_programs
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 2. customer_loyalty_accounts
CREATE TABLE IF NOT EXISTS customer_loyalty_accounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    program_id TEXT NOT NULL REFERENCES loyalty_programs(id),
    points_balance INTEGER NOT NULL DEFAULT 0,
    punches INTEGER NOT NULL DEFAULT 0,
    tier_name TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, customer_id, program_id)
);

CREATE INDEX IF NOT EXISTS idx_customer_loyalty_accounts_tenant_customer ON customer_loyalty_accounts(tenant_id, customer_id);

ALTER TABLE customer_loyalty_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts;
CREATE POLICY tenant_isolation_customer_loyalty_accounts
ON customer_loyalty_accounts
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 3. loyalty_transactions
CREATE TABLE IF NOT EXISTS loyalty_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    account_id TEXT NOT NULL REFERENCES customer_loyalty_accounts(id),
    transaction_type TEXT NOT NULL, -- earn, redeem
    points INTEGER NOT NULL DEFAULT 0,
    punches INTEGER NOT NULL DEFAULT 0,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_loyalty_transactions_tenant_account ON loyalty_transactions(tenant_id, account_id);

ALTER TABLE loyalty_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_transactions ON loyalty_transactions;
CREATE POLICY tenant_isolation_loyalty_transactions
ON loyalty_transactions
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 4. rewards
CREATE TABLE IF NOT EXISTS rewards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    program_id TEXT NOT NULL REFERENCES loyalty_programs(id),
    name TEXT NOT NULL,
    description TEXT,
    points_cost INTEGER NOT NULL DEFAULT 0,
    punches_cost INTEGER NOT NULL DEFAULT 0,
    reward_type TEXT NOT NULL, -- discount, free_item
    reward_value JSONB NOT NULL DEFAULT '{}'::jsonb,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_rewards_tenant_program ON rewards(tenant_id, program_id);

ALTER TABLE rewards ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_rewards ON rewards;
CREATE POLICY tenant_isolation_rewards
ON rewards
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
