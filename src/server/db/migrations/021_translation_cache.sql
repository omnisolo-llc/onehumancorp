CREATE TABLE IF NOT EXISTS translation_cache (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    text_hash TEXT NOT NULL,
    locale TEXT NOT NULL,
    translated_text TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_translation_cache_lookup ON translation_cache(tenant_id, text_hash, locale);

ALTER TABLE translation_cache ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'translation_cache'
          AND policyname = 'tenant_isolation_translation_cache'
    ) THEN
        CREATE POLICY tenant_isolation_translation_cache ON translation_cache
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
