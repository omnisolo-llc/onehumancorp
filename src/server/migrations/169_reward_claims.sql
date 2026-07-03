-- +goose Up

-- 1. Add lifetime_points to loyalty_ledger
ALTER TABLE IF EXISTS loyalty_ledger ADD COLUMN IF NOT EXISTS lifetime_points INTEGER DEFAULT 0;

-- 2. Create reward_claims table
CREATE TABLE IF NOT EXISTS reward_claims (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    discount_code TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_reward_claims_tenant_customer ON reward_claims(tenant_id, customer_id);

-- 3. Enable RLS on reward_claims
ALTER TABLE reward_claims ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_reward_claims ON reward_claims;
CREATE POLICY tenant_isolation_reward_claims
ON reward_claims
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_reward_claims ON reward_claims;
ALTER TABLE IF EXISTS reward_claims DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS reward_claims;

ALTER TABLE IF EXISTS loyalty_ledger DROP COLUMN IF EXISTS lifetime_points;
