CREATE TABLE IF NOT EXISTS social_posts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS email_campaigns (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    sent_count INT NOT NULL
);

CREATE TABLE IF NOT EXISTS referrals (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    referral_code TEXT NOT NULL,
    clicks INT DEFAULT 0,
    conversions INT DEFAULT 0,
    created_at_unix BIGINT NOT NULL
);
