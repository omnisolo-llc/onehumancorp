-- Unified Multilingual Hybrid Translation Mesh
-- GitHub Issue #24057

CREATE TABLE IF NOT EXISTS ohc_translation_cache (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    text_hash TEXT NOT NULL,
    target_locale TEXT NOT NULL,
    translated_text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, text_hash, target_locale)
);

CREATE INDEX IF NOT EXISTS idx_ohc_translation_cache_lookup ON ohc_translation_cache(tenant_id, text_hash, target_locale);

ALTER TABLE ohc_translation_cache ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_translation_cache ON ohc_translation_cache;

CREATE POLICY tenant_isolation_translation_cache
ON ohc_translation_cache
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
