CREATE TABLE IF NOT EXISTS social_media_posts (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    platform TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    scheduled_at TIMESTAMPTZ,
    posted_at TIMESTAMPTZ,
    created_at_unix BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS email_campaigns (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    name TEXT NOT NULL,
    subject TEXT NOT NULL,
    body TEXT NOT NULL,
    target_segment TEXT,
    status TEXT NOT NULL DEFAULT 'draft',
    emails_sent INTEGER DEFAULT 0,
    open_rate FLOAT DEFAULT 0.0,
    created_at_unix BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS business_milestones (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    milestone_type TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    reached BOOLEAN DEFAULT false,
    reached_at TIMESTAMPTZ,
    created_at_unix BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS storefront_visitors (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    visitor_id TEXT NOT NULL,
    page_url TEXT NOT NULL,
    referrer TEXT,
    created_at_unix BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS social_media_profiles (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    platform TEXT NOT NULL,
    connected_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
