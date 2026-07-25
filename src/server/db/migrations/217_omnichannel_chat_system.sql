-- +goose Up

CREATE TABLE IF NOT EXISTS omnichannel_tenants (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES omnichannel_tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_channels (
    id TEXT PRIMARY KEY,
    provider_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES omnichannel_tenants(id) ON DELETE CASCADE,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_conversations (
    id TEXT PRIMARY KEY,
    inbox_id TEXT NOT NULL REFERENCES omnichannel_inboxes(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES omnichannel_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    tenant_id TEXT NOT NULL REFERENCES omnichannel_tenants(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_messages (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL REFERENCES omnichannel_conversations(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    status TEXT NOT NULL,
    tenant_id TEXT NOT NULL REFERENCES omnichannel_tenants(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('omnichannel_tenants') IS NOT NULL THEN
        ALTER TABLE omnichannel_tenants ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omnichannel_tenants ON omnichannel_tenants USING (id::text = current_setting('app.current_tenant', true)) WITH CHECK (id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('omnichannel_inboxes') IS NOT NULL THEN
        ALTER TABLE omnichannel_inboxes ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omnichannel_inboxes ON omnichannel_inboxes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('omnichannel_contacts') IS NOT NULL THEN
        ALTER TABLE omnichannel_contacts ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omnichannel_contacts ON omnichannel_contacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('omnichannel_conversations') IS NOT NULL THEN
        ALTER TABLE omnichannel_conversations ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omnichannel_conversations ON omnichannel_conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('omnichannel_messages') IS NOT NULL THEN
        ALTER TABLE omnichannel_messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omnichannel_messages ON omnichannel_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_omnichannel_messages ON omnichannel_messages;
    DROP POLICY IF EXISTS tenant_isolation_omnichannel_conversations ON omnichannel_conversations;
    DROP POLICY IF EXISTS tenant_isolation_omnichannel_contacts ON omnichannel_contacts;
    DROP POLICY IF EXISTS tenant_isolation_omnichannel_inboxes ON omnichannel_inboxes;
    DROP POLICY IF EXISTS tenant_isolation_omnichannel_tenants ON omnichannel_tenants;
END
$$;

DROP TABLE IF EXISTS omnichannel_messages CASCADE;
DROP TABLE IF EXISTS omnichannel_conversations CASCADE;
DROP TABLE IF EXISTS omnichannel_contacts CASCADE;
DROP TABLE IF EXISTS omnichannel_channels CASCADE;
DROP TABLE IF EXISTS omnichannel_inboxes CASCADE;
DROP TABLE IF EXISTS omnichannel_tenants CASCADE;
