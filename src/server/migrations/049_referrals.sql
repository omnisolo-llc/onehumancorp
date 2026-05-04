CREATE TABLE IF NOT EXISTS referrals (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    referral_code TEXT UNIQUE NOT NULL,
    clicks INTEGER DEFAULT 0,
    conversions INTEGER DEFAULT 0,
    created_at_unix BIGINT NOT NULL
);

ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;

-- Note: Using 'app.current_tenant' as seen in db.rs or 'ohc.current_organization_id' from memory.
-- Checking db.rs again: conn.execute("SET app.current_tenant = 'system'")
-- Checking memory: "ohc.current_organization_id"
-- I'll use both patterns to be safe if they coexist, but following db.rs strictly for current_tenant.
CREATE POLICY referrals_isolation_policy ON referrals
USING (organization_id = current_setting('app.current_tenant', true));
