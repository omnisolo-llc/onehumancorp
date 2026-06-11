-- +goose Up
-- Migration 036: Loyalty Wallet and Reward Ledger

CREATE TABLE IF NOT EXISTS loyalty_wallet (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    points_balance INTEGER DEFAULT 0,
    tier_name TEXT,
    last_updated TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, customer_id)
);

CREATE INDEX IF NOT EXISTS idx_loyalty_wallet_tenant_customer ON loyalty_wallet(tenant_id, customer_id);

DO $$
BEGIN
    IF to_regclass('loyalty_wallet') IS NOT NULL THEN
        ALTER TABLE loyalty_wallet ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_loyalty_wallet ON loyalty_wallet USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS reward_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    points_change INTEGER NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_reward_ledger_tenant_customer ON reward_ledger(tenant_id, customer_id);

DO $$
BEGIN
    IF to_regclass('reward_ledger') IS NOT NULL THEN
        ALTER TABLE reward_ledger ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_reward_ledger ON reward_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DROP TABLE IF EXISTS reward_ledger CASCADE;
DROP TABLE IF EXISTS loyalty_wallet CASCADE;
