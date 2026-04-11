-- 032_growth_referrals.sql
-- Table for tracking referral links

CREATE TABLE IF NOT EXISTS referral_links (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    code TEXT UNIQUE NOT NULL,
    uses_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_referral_links_user_id ON referral_links(user_id);
