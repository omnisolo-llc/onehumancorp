-- Migration: 003_growth_features.sql
-- Growth-oriented features: Referrals, Viral Loop, Social Posting, Email Marketing, Quotas.

-- 1. Business Metadata for Shareable Link Cards (OpenGraph)
CREATE TABLE IF NOT EXISTS business_metadata (
    tenant_id TEXT PRIMARY KEY REFERENCES tenants(id) ON DELETE CASCADE,
    tagline TEXT,
    description TEXT,
    logo_url TEXT,
    cover_image_url TEXT,
    social_links JSONB DEFAULT '{}',
    opengraph_metadata JSONB DEFAULT '{}',
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 2. Referral Rewards & Credit Attribution
CREATE TABLE IF NOT EXISTS referral_rewards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    referral_id TEXT REFERENCES referrals(id) ON DELETE SET NULL,
    reward_type TEXT NOT NULL, -- 'PRO_MONTH_FREE', 'CREDIT'
    status TEXT DEFAULT 'PENDING', -- 'PENDING', 'APPLIED', 'EXPIRED'
    amount_cents BIGINT DEFAULT 0,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    applied_at TIMESTAMPTZ
);

-- 3. Social Media Auto-Posting AI Feature
CREATE TABLE IF NOT EXISTS social_accounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    platform TEXT NOT NULL, -- 'instagram', 'facebook', 'x', 'whatsapp'
    account_name TEXT,
    access_token TEXT, -- Encrypted in real scenarios
    status TEXT DEFAULT 'CONNECTED',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS social_posts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    platform TEXT NOT NULL,
    content TEXT NOT NULL,
    media_urls TEXT[] DEFAULT '{}',
    status TEXT DEFAULT 'DRAFT', -- 'DRAFT', 'SCHEDULED', 'PUBLISHED', 'FAILED'
    scheduled_at TIMESTAMPTZ,
    published_at TIMESTAMPTZ,
    approval_task_id TEXT, -- References shared_tasks
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 4. Simple Email Marketing
CREATE TABLE IF NOT EXISTS email_campaigns (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    subject TEXT NOT NULL,
    content_html TEXT NOT NULL,
    status TEXT DEFAULT 'DRAFT', -- 'DRAFT', 'SENDING', 'SENT', 'FAILED'
    template_id TEXT,
    total_recipients INTEGER DEFAULT 0,
    open_count INTEGER DEFAULT 0,
    click_count INTEGER DEFAULT 0,
    scheduled_at TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 5. Success Milestones tracking
CREATE TABLE IF NOT EXISTS business_milestones (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    milestone_key TEXT NOT NULL, -- '1ST_SALE', '10TH_ORDER', '100_VISITORS'
    achieved_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    notified BOOLEAN DEFAULT FALSE,
    UNIQUE(tenant_id, milestone_key)
);

-- 6. Add soft paywall/tier info to tenants if missing or update
-- (Tiers already exist in tenants table: 'free', 'starter', 'pro', 'business')
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS actions_count_current_month INTEGER DEFAULT 0;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS last_reset_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;

-- 7. Add referral tracking to referrals table if needed
-- (The referrals table in 002 already has clicks and conversions)
ALTER TABLE referrals ADD COLUMN IF NOT EXISTS channel TEXT;
ALTER TABLE referrals ADD COLUMN IF NOT EXISTS metadata JSONB DEFAULT '{}';

-- 8. Viral loops: track where the user came from
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS referred_by_code TEXT;

-- 9. Add OpenGraph policy
ALTER TABLE business_metadata ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_business_metadata ON business_metadata USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE social_posts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_social_posts ON social_posts USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE email_campaigns ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_email_campaigns ON email_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE referral_rewards ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_referral_rewards ON referral_rewards USING (tenant_id::text = current_setting('app.current_tenant', true));
