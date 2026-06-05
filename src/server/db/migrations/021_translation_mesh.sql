CREATE TABLE IF NOT EXISTS translation_cache (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source_text_hash TEXT NOT NULL,
    source_lang TEXT NOT NULL,
    target_lang TEXT NOT NULL,
    translated_text TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, source_text_hash, target_lang)
);

CREATE INDEX IF NOT EXISTS idx_translation_cache_lookup ON translation_cache(tenant_id, source_text_hash, target_lang);
