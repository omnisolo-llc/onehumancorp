-- +goose Up

CREATE TABLE IF NOT EXISTS referral_codes (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID NOT NULL,
    code TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, code),
    UNIQUE(tenant_id, customer_id)
);

CREATE TABLE IF NOT EXISTS referrals (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    referral_code_id UUID NOT NULL REFERENCES referral_codes(id) ON DELETE RESTRICT,
    referred_customer_id UUID,
    status TEXT NOT NULL CHECK (status IN ('Clicked', 'SignedUp', 'Purchased')),
    reward_issued BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS and setup tenant isolation policies
ALTER TABLE referral_codes ENABLE ROW LEVEL SECURITY;
ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_referral_codes ON referral_codes;
CREATE POLICY tenant_isolation_referral_codes ON referral_codes
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
CREATE POLICY tenant_isolation_referrals ON referrals
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS referrals CASCADE;
DROP TABLE IF EXISTS referral_codes CASCADE;
