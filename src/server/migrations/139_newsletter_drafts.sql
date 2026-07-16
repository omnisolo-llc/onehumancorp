CREATE TABLE IF NOT EXISTS newsletter_drafts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    subject TEXT NOT NULL,
    body_html TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    sent_at TIMESTAMP WITH TIME ZONE
);

ALTER TABLE newsletter_drafts ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy ON newsletter_drafts
    USING (tenant_id = current_setting('app.current_tenant')::UUID)
WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE INDEX IF NOT EXISTS idx_newsletter_drafts_tenant_id ON newsletter_drafts(tenant_id);
