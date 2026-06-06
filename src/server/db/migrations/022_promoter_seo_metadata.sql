CREATE TABLE IF NOT EXISTS ohc_seo_metadata (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL, -- 'product', 'service', 'business_profile', etc.
    meta_title TEXT,
    meta_description TEXT,
    structured_data JSONB,
    generated_keywords TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING_APPROVAL', -- 'PENDING_APPROVAL', 'APPROVED', 'REJECTED'
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, entity_id, entity_type)
);

CREATE INDEX IF NOT EXISTS idx_seo_metadata_tenant_entity ON ohc_seo_metadata(tenant_id, entity_type, entity_id);

ALTER TABLE ohc_seo_metadata ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_seo_metadata ON ohc_seo_metadata;
CREATE POLICY tenant_isolation_ohc_seo_metadata
ON ohc_seo_metadata
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
