-- Migration to add translation_cache for Unified Multilingual Hybrid Translation Mesh
CREATE TABLE IF NOT EXISTS translation_cache (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    text_hash TEXT NOT NULL,
    source_lang TEXT NOT NULL,
    target_lang TEXT NOT NULL,
    translated_text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, text_hash, target_lang)
);

CREATE INDEX IF NOT EXISTS idx_translation_cache_lookup ON translation_cache(tenant_id, text_hash, target_lang);

ALTER TABLE translation_cache ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_translation_cache ON translation_cache;
CREATE POLICY tenant_isolation_translation_cache
ON translation_cache
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
