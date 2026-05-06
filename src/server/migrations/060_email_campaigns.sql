CREATE TABLE IF NOT EXISTS email_campaigns (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    template_name TEXT NOT NULL,
    preview_text TEXT,
    emails_sent INT NOT NULL DEFAULT 0,
    open_rate TEXT,
    status TEXT NOT NULL
);

ALTER TABLE email_campaigns ENABLE ROW LEVEL SECURITY;
CREATE POLICY email_campaigns_isolation_policy ON email_campaigns USING (organization_id = current_setting('app.current_tenant', true));
