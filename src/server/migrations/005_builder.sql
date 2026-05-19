CREATE TABLE IF NOT EXISTS builder_sites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    domain VARCHAR(255),
    published_at TIMESTAMP WITH TIME ZONE,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS builder_pages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    site_id UUID NOT NULL REFERENCES builder_sites(id) ON DELETE CASCADE,
    path VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    seo_metadata JSONB DEFAULT '{}'::jsonb,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS builder_blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    page_id UUID NOT NULL REFERENCES builder_pages(id) ON DELETE CASCADE,
    block_type VARCHAR(255) NOT NULL,
    content JSONB NOT NULL DEFAULT '{}'::jsonb,
    sort_order INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'builder_sites', 'builder_pages', 'builder_blocks'
        ])
    LOOP
        EXECUTE format('ALTER TABLE IF EXISTS %I ENABLE ROW LEVEL SECURITY', t_name);
        EXECUTE format('ALTER TABLE IF EXISTS %I FORCE ROW LEVEL SECURITY', t_name);

        pol_name := format('tenant_isolation_%s', t_name);

        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE policyname = pol_name AND tablename = t_name
        ) THEN
            EXECUTE format('CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true))', pol_name, t_name);
        END IF;
    END LOOP;
END
$$;
