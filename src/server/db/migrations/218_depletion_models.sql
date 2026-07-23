-- +goose Up

CREATE TABLE IF NOT EXISTS depletion_models (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    raw_material_id TEXT NOT NULL REFERENCES raw_materials(id) ON DELETE CASCADE,
    burn_rate_per_day DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    confidence_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_depletion_models_tenant ON depletion_models(tenant_id);
CREATE INDEX IF NOT EXISTS idx_depletion_models_raw_material ON depletion_models(raw_material_id);

ALTER TABLE depletion_models ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_depletion_models ON depletion_models;
CREATE POLICY tenant_isolation_depletion_models
ON depletion_models
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS agent_reorder_intents (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    raw_material_id TEXT NOT NULL REFERENCES raw_materials(id) ON DELETE CASCADE,
    suggested_quantity INTEGER NOT NULL,
    vendor_id TEXT REFERENCES vendors(id),
    status TEXT NOT NULL DEFAULT 'DRAFT',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_reorder_intents_tenant ON agent_reorder_intents(tenant_id);

ALTER TABLE agent_reorder_intents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_reorder_intents ON agent_reorder_intents;
CREATE POLICY tenant_isolation_agent_reorder_intents
ON agent_reorder_intents
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS agent_reorder_intents;
DROP TABLE IF EXISTS depletion_models;
