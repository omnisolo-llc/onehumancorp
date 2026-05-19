-- Migration 005: Website Builder Tables
CREATE TABLE IF NOT EXISTS builder_sites (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    domain TEXT UNIQUE,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS builder_pages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    site_id UUID NOT NULL REFERENCES builder_sites(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    title TEXT NOT NULL,
    seo_metadata JSONB DEFAULT '{}',
    is_published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(site_id, path)
);

CREATE TABLE IF NOT EXISTS builder_blocks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    page_id UUID NOT NULL REFERENCES builder_pages(id) ON DELETE CASCADE,
    block_type TEXT NOT NULL,
    content JSONB DEFAULT '{}',
    sort_order INTEGER NOT NULL,
    is_published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE builder_sites ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_sites FORCE ROW LEVEL SECURITY;
ALTER TABLE builder_pages ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_pages FORCE ROW LEVEL SECURITY;
ALTER TABLE builder_blocks ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_blocks FORCE ROW LEVEL SECURITY;

-- RLS Policies
CREATE POLICY tenant_isolation_builder_sites ON builder_sites
    USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_builder_pages ON builder_pages
    USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_builder_blocks ON builder_blocks
    USING (tenant_id::text = current_setting('app.current_tenant', true));
