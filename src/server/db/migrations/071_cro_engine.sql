-- Autonomous Generative Conversion Rate Optimization (CRO) Engine
-- GitHub Issue #23499

CREATE TABLE IF NOT EXISTS cro_experiments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    site_id UUID NOT NULL REFERENCES builder_sites(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    target_element TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'running',
    winning_variant_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS cro_variants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    experiment_id UUID NOT NULL REFERENCES cro_experiments(id) ON DELETE CASCADE,
    variant_name TEXT NOT NULL,
    content JSONB NOT NULL DEFAULT '{}'::jsonb,
    traffic_weight FLOAT NOT NULL DEFAULT 1.0,
    views INTEGER NOT NULL DEFAULT 0,
    conversions INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE cro_experiments ENABLE ROW LEVEL SECURITY;
ALTER TABLE cro_experiments FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_cro_experiments ON cro_experiments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE cro_variants ENABLE ROW LEVEL SECURITY;
ALTER TABLE cro_variants FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_cro_variants ON cro_variants USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
