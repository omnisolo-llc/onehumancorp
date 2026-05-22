-- Migration: 008_builder.sql

CREATE TABLE IF NOT EXISTS builder_sites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    domain TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS builder_pages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    site_id UUID NOT NULL REFERENCES builder_sites(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    title TEXT NOT NULL,
    seo_metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS builder_blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    page_id UUID NOT NULL REFERENCES builder_pages(id) ON DELETE CASCADE,
    block_type TEXT NOT NULL,
    content JSONB NOT NULL DEFAULT '{}'::jsonb,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- RLS for builder_sites
ALTER TABLE builder_sites ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_sites FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_builder_sites ON builder_sites USING (tenant_id::text = current_setting('app.current_tenant', true));

-- RLS for builder_pages
ALTER TABLE builder_pages ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_pages FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_builder_pages ON builder_pages USING (tenant_id::text = current_setting('app.current_tenant', true));

-- RLS for builder_blocks
ALTER TABLE builder_blocks ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_blocks FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_builder_blocks ON builder_blocks USING (tenant_id::text = current_setting('app.current_tenant', true));
