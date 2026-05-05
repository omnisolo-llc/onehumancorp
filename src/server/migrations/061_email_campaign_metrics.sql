CREATE TABLE IF NOT EXISTS email_campaign_metrics (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    campaign_id TEXT NOT NULL,
    recipient TEXT NOT NULL,
    opened BOOLEAN DEFAULT FALSE,
    clicked BOOLEAN DEFAULT FALSE,
    sent_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    opened_at TIMESTAMP WITH TIME ZONE,
    clicked_at TIMESTAMP WITH TIME ZONE,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

ALTER TABLE email_campaign_metrics ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy ON email_campaign_metrics
    USING (tenant_id = current_setting('app.current_tenant')::text);

CREATE INDEX IF NOT EXISTS idx_email_campaign_metrics_tenant ON email_campaign_metrics(tenant_id, campaign_id);
