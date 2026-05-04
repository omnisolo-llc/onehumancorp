CREATE TABLE IF NOT EXISTS builder_sites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    domain VARCHAR(255) UNIQUE,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE builder_sites ENABLE ROW LEVEL SECURITY;
CREATE POLICY builder_sites_tenant_policy ON builder_sites FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE TABLE IF NOT EXISTS builder_pages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    site_id UUID NOT NULL REFERENCES builder_sites(id) ON DELETE CASCADE,
    path VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    seo_metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(site_id, path)
);
ALTER TABLE builder_pages ENABLE ROW LEVEL SECURITY;
CREATE POLICY builder_pages_tenant_policy ON builder_pages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE TABLE IF NOT EXISTS builder_blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    page_id UUID NOT NULL REFERENCES builder_pages(id) ON DELETE CASCADE,
    block_type VARCHAR(50) NOT NULL,
    content JSONB NOT NULL DEFAULT '{}'::jsonb,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE builder_blocks ENABLE ROW LEVEL SECURITY;
CREATE POLICY builder_blocks_tenant_policy ON builder_blocks FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);
