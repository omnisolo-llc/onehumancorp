-- +goose Up
CREATE TABLE IF NOT EXISTS referrals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    referral_code TEXT UNIQUE NOT NULL,
    clicks INTEGER NOT NULL DEFAULT 0,
    conversions INTEGER NOT NULL DEFAULT 0,
    created_at_unix BIGINT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_referrals_tenant_id ON referrals(tenant_id);
CREATE INDEX IF NOT EXISTS idx_referrals_user_id ON referrals(user_id);
CREATE INDEX IF NOT EXISTS idx_referrals_code ON referrals(referral_code);

DO $$
BEGIN
    IF to_regclass('referrals') IS NOT NULL THEN
        ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
        CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
END
$$;

DROP TABLE IF EXISTS referrals CASCADE;
