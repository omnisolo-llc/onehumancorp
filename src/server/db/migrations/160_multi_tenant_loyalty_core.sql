-- +goose Up
CREATE TABLE IF NOT EXISTS loyalty_programs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    program_type TEXT NOT NULL, -- points, punch_card, tiers
    config JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_loyalty_programs_tenant ON loyalty_programs(tenant_id);

CREATE TABLE IF NOT EXISTS customer_loyalty_accounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    program_id TEXT NOT NULL REFERENCES loyalty_programs(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
    points_balance INTEGER DEFAULT 0,
    punches INTEGER DEFAULT 0,
    tier_name TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, program_id, customer_id)
);
CREATE INDEX IF NOT EXISTS idx_customer_loyalty_accounts_tenant_customer ON customer_loyalty_accounts(tenant_id, customer_id);

CREATE TABLE IF NOT EXISTS loyalty_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    account_id TEXT NOT NULL REFERENCES customer_loyalty_accounts(id) ON DELETE CASCADE,
    transaction_type TEXT NOT NULL, -- earn, redeem, adjust
    amount INTEGER NOT NULL,
    reason TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_loyalty_transactions_tenant_account ON loyalty_transactions(tenant_id, account_id);

CREATE TABLE IF NOT EXISTS loyalty_rewards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    program_id TEXT NOT NULL REFERENCES loyalty_programs(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    cost_in_points INTEGER NOT NULL,
    reward_type TEXT NOT NULL, -- discount, free_item
    reward_value JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_loyalty_rewards_tenant_program ON loyalty_rewards(tenant_id, program_id);

DO $$
BEGIN
    IF to_regclass('loyalty_programs') IS NOT NULL THEN
        ALTER TABLE loyalty_programs ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_loyalty_programs ON loyalty_programs;
        CREATE POLICY tenant_isolation_loyalty_programs ON loyalty_programs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('customer_loyalty_accounts') IS NOT NULL THEN
        ALTER TABLE customer_loyalty_accounts ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts;
        CREATE POLICY tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('loyalty_transactions') IS NOT NULL THEN
        ALTER TABLE loyalty_transactions ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_loyalty_transactions ON loyalty_transactions;
        CREATE POLICY tenant_isolation_loyalty_transactions ON loyalty_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('loyalty_rewards') IS NOT NULL THEN
        ALTER TABLE loyalty_rewards ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_loyalty_rewards ON loyalty_rewards;
        CREATE POLICY tenant_isolation_loyalty_rewards ON loyalty_rewards USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_loyalty_programs ON loyalty_programs;
    DROP POLICY IF EXISTS tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts;
    DROP POLICY IF EXISTS tenant_isolation_loyalty_transactions ON loyalty_transactions;
    DROP POLICY IF EXISTS tenant_isolation_loyalty_rewards ON loyalty_rewards;
END
$$;

DROP TABLE IF EXISTS loyalty_rewards CASCADE;
DROP TABLE IF EXISTS loyalty_transactions CASCADE;
DROP TABLE IF EXISTS customer_loyalty_accounts CASCADE;
DROP TABLE IF EXISTS loyalty_programs CASCADE;
