CREATE TABLE IF NOT EXISTS ohc_translation_preferences (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    target_languages JSONB NOT NULL DEFAULT '[]'::jsonb, -- Array of language codes, e.g., ["ar", "es"]
    auto_translate BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id)
);

CREATE INDEX IF NOT EXISTS idx_ohc_translation_prefs_tenant ON ohc_translation_preferences(tenant_id);

ALTER TABLE ohc_translation_preferences ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_translation_preferences ON ohc_translation_preferences;
CREATE POLICY tenant_isolation_ohc_translation_preferences
ON ohc_translation_preferences
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
