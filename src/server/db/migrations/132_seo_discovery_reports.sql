-- Migration 132: SEO Discovery Reports
CREATE TABLE IF NOT EXISTS seo_discovery_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    month VARCHAR(255) NOT NULL,
    plain_language_summary TEXT NOT NULL,
    metrics JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_seo_discovery_reports_tenant_id ON seo_discovery_reports(tenant_id);

ALTER TABLE seo_discovery_reports ENABLE ROW LEVEL SECURITY;
ALTER TABLE seo_discovery_reports FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_seo_discovery_reports ON seo_discovery_reports
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
