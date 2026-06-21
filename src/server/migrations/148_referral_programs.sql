CREATE TABLE IF NOT EXISTS referral_codes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    code TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, code),
    UNIQUE(tenant_id, customer_id)
);
CREATE INDEX IF NOT EXISTS idx_referral_codes_tenant ON referral_codes(tenant_id);
ALTER TABLE referral_codes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referral_codes ON referral_codes;
CREATE POLICY tenant_isolation_referral_codes
ON referral_codes
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS referrals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    referral_code_id TEXT NOT NULL REFERENCES referral_codes(id) ON DELETE RESTRICT,
    referred_customer_id TEXT,
    status TEXT NOT NULL CHECK (status IN ('Clicked', 'SignedUp', 'Purchased')),
    reward_issued BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_referrals_tenant ON referrals(tenant_id);
ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
CREATE POLICY tenant_isolation_referrals
ON referrals
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
