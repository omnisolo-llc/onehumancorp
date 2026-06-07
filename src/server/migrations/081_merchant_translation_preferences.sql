-- Autonomous Multi-Language Edge Translation Architecture
-- GitHub Issue #24766

CREATE TABLE IF NOT EXISTS merchant_translation_preferences (
    tenant_id TEXT PRIMARY KEY REFERENCES tenants(id) ON DELETE CASCADE,
    primary_language TEXT NOT NULL DEFAULT 'en',
    enabled_languages TEXT[] NOT NULL DEFAULT '{}',
    auto_translate BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE merchant_translation_preferences ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_merchant_translation_preferences ON merchant_translation_preferences;
CREATE POLICY tenant_isolation_merchant_translation_preferences
ON merchant_translation_preferences
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
