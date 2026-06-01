-- Migration 056: Multilingual Localization Mesh

CREATE TABLE IF NOT EXISTS locale_configs (
    tenant_id TEXT PRIMARY KEY REFERENCES tenants(id) ON DELETE CASCADE,
    primary_locale TEXT NOT NULL,
    supported_locales JSONB NOT NULL DEFAULT '[]'::jsonb,
    auto_translate BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE locale_configs ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_locale_configs ON locale_configs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS localized_contents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    locale TEXT NOT NULL,
    localized_name TEXT,
    localized_desc TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, entity_id, entity_type, locale)
);
ALTER TABLE localized_contents ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_localized_contents ON localized_contents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS conversation_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    conversation_id TEXT NOT NULL,
    original_text TEXT NOT NULL,
    original_locale TEXT NOT NULL,
    translated_text TEXT,
    target_locale TEXT,
    sender_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE conversation_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_conversation_messages ON conversation_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
