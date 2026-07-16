-- Migration 132_e: SEO Pre-render Jobs
CREATE TABLE IF NOT EXISTS seo_pre_render_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    site_id UUID,
    page_path VARCHAR(2048) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, processing, completed, failed
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_seo_pre_render_jobs_tenant_id ON seo_pre_render_jobs(tenant_id);
CREATE INDEX IF NOT EXISTS idx_seo_pre_render_jobs_status_created ON seo_pre_render_jobs(status, created_at);

ALTER TABLE seo_pre_render_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE seo_pre_render_jobs FORCE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_seo_pre_render_jobs ON seo_pre_render_jobs
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
