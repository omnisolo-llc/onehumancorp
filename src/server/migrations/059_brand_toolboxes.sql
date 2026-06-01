-- Migration 059: Brand DNA toolbox persistence

CREATE TABLE IF NOT EXISTS builder_brand_toolboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    source_description TEXT NOT NULL DEFAULT '',
    toolbox JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_builder_brand_toolboxes_tenant_id
    ON builder_brand_toolboxes(tenant_id);

ALTER TABLE builder_brand_toolboxes ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_brand_toolboxes FORCE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'builder_brand_toolboxes'
          AND policyname = 'tenant_isolation_builder_brand_toolboxes'
    ) THEN
        CREATE POLICY tenant_isolation_builder_brand_toolboxes
            ON builder_brand_toolboxes
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
