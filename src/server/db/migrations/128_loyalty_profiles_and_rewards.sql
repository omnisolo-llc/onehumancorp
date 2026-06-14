-- +goose Up
CREATE TABLE IF NOT EXISTS loyalty_profiles (
    customer_id UUID PRIMARY KEY REFERENCES customers(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    lifetime_value_cents BIGINT NOT NULL DEFAULT 0,
    purchase_frequency INT NOT NULL DEFAULT 0,
    last_purchase_date TIMESTAMPTZ,
    credits INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE loyalty_profiles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_loyalty_profiles ON loyalty_profiles USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS reward_ledger (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL, -- e.g. EARNED, REDEEMED
    credits_change INT NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE reward_ledger ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_reward_ledger ON reward_ledger USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_reward_ledger ON reward_ledger;
DROP TABLE IF EXISTS reward_ledger CASCADE;

DROP POLICY IF EXISTS tenant_isolation_loyalty_profiles ON loyalty_profiles;
DROP TABLE IF EXISTS loyalty_profiles CASCADE;
