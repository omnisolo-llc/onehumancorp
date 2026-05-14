CREATE TABLE IF NOT EXISTS email_campaigns (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    template_name TEXT,
    subject TEXT,
    body TEXT,
    audience_type TEXT,
    emails_sent INTEGER DEFAULT 0,
    open_rate FLOAT DEFAULT 0.0,
    created_at_unix BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS social_posts (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    platform TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT DEFAULT 'PENDING',
    scheduled_for_unix BIGINT,
    created_at_unix BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS milestone_notifications (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    milestone_type TEXT NOT NULL,
    message TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    created_at_unix BIGINT NOT NULL
);

ALTER TABLE email_campaigns ENABLE ROW LEVEL SECURITY;
CREATE POLICY email_campaigns_isolation_policy ON email_campaigns
USING (organization_id = current_setting('app.current_tenant', true));

ALTER TABLE social_posts ENABLE ROW LEVEL SECURITY;
CREATE POLICY social_posts_isolation_policy ON social_posts
USING (organization_id = current_setting('app.current_tenant', true));

ALTER TABLE milestone_notifications ENABLE ROW LEVEL SECURITY;
CREATE POLICY milestone_notifications_isolation_policy ON milestone_notifications
USING (organization_id = current_setting('app.current_tenant', true));
