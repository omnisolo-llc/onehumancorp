-- +goose Up

CREATE TABLE IF NOT EXISTS depletion_models (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    raw_material_id TEXT NOT NULL REFERENCES raw_materials(id) ON DELETE CASCADE,
    burn_rate_per_day DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    confidence_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_depletion_models_tenant ON depletion_models(tenant_id);

DO $$
BEGIN
    IF to_regclass('depletion_models') IS NOT NULL THEN
        ALTER TABLE depletion_models ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'depletion_models'
                AND policyname = 'tenant_isolation_depletion_models'
        ) THEN
            CREATE POLICY tenant_isolation_depletion_models ON depletion_models USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;


CREATE TABLE IF NOT EXISTS agent_reorder_intents (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    raw_material_id TEXT NOT NULL REFERENCES raw_materials(id) ON DELETE CASCADE,
    suggested_quantity INTEGER NOT NULL,
    vendor_id TEXT REFERENCES vendors(id),
    status TEXT NOT NULL DEFAULT 'DRAFT',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_reorder_intents_tenant ON agent_reorder_intents(tenant_id);

DO $$
BEGIN
    IF to_regclass('agent_reorder_intents') IS NOT NULL THEN
        ALTER TABLE agent_reorder_intents ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'agent_reorder_intents'
                AND policyname = 'tenant_isolation_agent_reorder_intents'
        ) THEN
            CREATE POLICY tenant_isolation_agent_reorder_intents ON agent_reorder_intents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;
