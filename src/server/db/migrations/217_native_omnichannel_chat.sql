-- +goose Up
-- Migration 217: Native Omnichannel Chat

CREATE TABLE IF NOT EXISTS chat_inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_channels (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
    channel_type TEXT NOT NULL,
    config TEXT DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES chat_contacts(id) ON DELETE CASCADE,
    assignee_id TEXT,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL, -- e.g. 'contact', 'agent', 'bot'
    sender_id TEXT,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('chat_inboxes') IS NOT NULL THEN
        ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;
        CREATE POLICY chat_inboxes_tenant_isolation_policy ON chat_inboxes FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_channels') IS NOT NULL THEN
        ALTER TABLE chat_channels ENABLE ROW LEVEL SECURITY;
        CREATE POLICY chat_channels_tenant_isolation_policy ON chat_channels FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_contacts') IS NOT NULL THEN
        ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
        CREATE POLICY chat_contacts_tenant_isolation_policy ON chat_contacts FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_conversations') IS NOT NULL THEN
        ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
        CREATE POLICY chat_conversations_tenant_isolation_policy ON chat_conversations FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_messages') IS NOT NULL THEN
        ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY chat_messages_tenant_isolation_policy ON chat_messages FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS chat_inboxes_tenant_isolation_policy ON chat_inboxes;
    DROP POLICY IF EXISTS chat_channels_tenant_isolation_policy ON chat_channels;
    DROP POLICY IF EXISTS chat_contacts_tenant_isolation_policy ON chat_contacts;
    DROP POLICY IF EXISTS chat_conversations_tenant_isolation_policy ON chat_conversations;
    DROP POLICY IF EXISTS chat_messages_tenant_isolation_policy ON chat_messages;
END
$$;

DROP TABLE IF EXISTS chat_messages CASCADE;
DROP TABLE IF EXISTS chat_conversations CASCADE;
DROP TABLE IF EXISTS chat_contacts CASCADE;
DROP TABLE IF EXISTS chat_channels CASCADE;
DROP TABLE IF EXISTS chat_inboxes CASCADE;
