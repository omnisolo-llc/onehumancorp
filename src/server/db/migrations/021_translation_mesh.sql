-- Unified Multilingual Hybrid Translation Mesh
-- GitHub Issue #24057

CREATE TABLE IF NOT EXISTS ohc_translation_cache (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source_hash TEXT NOT NULL,
    source_text TEXT NOT NULL,
    target_locale TEXT NOT NULL,
    translated_text TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING', -- PENDING, COMPLETED, FAILED
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, source_hash, target_locale)
);

CREATE INDEX IF NOT EXISTS idx_ohc_translation_cache_lookup
ON ohc_translation_cache(tenant_id, source_hash, target_locale);

ALTER TABLE ohc_translation_cache ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_translation_cache ON ohc_translation_cache;
CREATE POLICY tenant_isolation_ohc_translation_cache
ON ohc_translation_cache
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- SQLite equivalent (usually handled in code but good to document if needed)
-- CREATE TABLE IF NOT EXISTS ohc_translation_cache ( ... );
