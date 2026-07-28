-- +goose Up
CREATE TABLE IF NOT EXISTS omnichannel_inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES omnichannel_inboxes(id) ON DELETE CASCADE,
    contact_id TEXT REFERENCES omnichannel_contacts(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'open',
    channel TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES omnichannel_conversations(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    message_type TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    sender_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('omnichannel_inboxes') IS NOT NULL THEN
        ALTER TABLE omnichannel_inboxes ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omni_inboxes ON omnichannel_inboxes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('omnichannel_contacts') IS NOT NULL THEN
        ALTER TABLE omnichannel_contacts ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omni_contacts ON omnichannel_contacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('omnichannel_conversations') IS NOT NULL THEN
        ALTER TABLE omnichannel_conversations ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omni_conversations ON omnichannel_conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('omnichannel_messages') IS NOT NULL THEN
        ALTER TABLE omnichannel_messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omni_messages ON omnichannel_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_omni_inboxes ON omnichannel_inboxes;
    DROP POLICY IF EXISTS tenant_isolation_omni_contacts ON omnichannel_contacts;
    DROP POLICY IF EXISTS tenant_isolation_omni_conversations ON omnichannel_conversations;
    DROP POLICY IF EXISTS tenant_isolation_omni_messages ON omnichannel_messages;
END
$$;

DROP TABLE IF EXISTS omnichannel_messages CASCADE;
DROP TABLE IF EXISTS omnichannel_conversations CASCADE;
DROP TABLE IF EXISTS omnichannel_contacts CASCADE;
DROP TABLE IF EXISTS omnichannel_inboxes CASCADE;
