CREATE TABLE IF NOT EXISTS user_referrals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    referral_code VARCHAR(255) NOT NULL UNIQUE,
    invites_sent INT DEFAULT 0,
    invites_accepted INT DEFAULT 0,
    pro_credits_earned INT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE user_referrals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON user_referrals USING (tenant_id = current_setting('app.current_tenant', true));
