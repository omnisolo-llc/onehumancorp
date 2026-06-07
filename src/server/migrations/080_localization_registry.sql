-- Autonomous Multi-Language Edge Translation Architecture
-- GitHub Issue #24766

CREATE TABLE IF NOT EXISTS localization_registry (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    resource_id TEXT NOT NULL,
    resource_type TEXT NOT NULL, -- e.g., 'product', 'category', 'storefront'
    language_code TEXT NOT NULL,
    translated_text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, resource_id, resource_type, language_code)
);

CREATE INDEX IF NOT EXISTS idx_localization_registry_tenant_resource ON localization_registry(tenant_id, resource_id, language_code);
CREATE INDEX IF NOT EXISTS idx_localization_registry_edge_lookup ON localization_registry(tenant_id, language_code);

ALTER TABLE localization_registry ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_localization_registry ON localization_registry;
CREATE POLICY tenant_isolation_localization_registry
ON localization_registry
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
