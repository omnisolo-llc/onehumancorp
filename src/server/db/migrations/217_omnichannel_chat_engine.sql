-- +goose Up
-- Migration 217: Native Rust Omnichannel Chat (Inbox, Contact, Conversation, Message)

CREATE TABLE IF NOT EXISTS chat_inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
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
    status TEXT NOT NULL DEFAULT 'open',
    channel TEXT NOT NULL DEFAULT 'web',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL, -- 'agent', 'customer', 'bot'
    sender_id TEXT, -- e.g., agent user_id or contact_id
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'sent',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('chat_inboxes') IS NOT NULL THEN
        ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_chat_inboxes ON chat_inboxes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_contacts') IS NOT NULL THEN
        ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_chat_contacts ON chat_contacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_conversations') IS NOT NULL THEN
        ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_chat_conversations ON chat_conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_messages') IS NOT NULL THEN
        ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_chat_messages ON chat_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_chat_messages ON chat_messages;
    DROP POLICY IF EXISTS tenant_isolation_chat_conversations ON chat_conversations;
    DROP POLICY IF EXISTS tenant_isolation_chat_contacts ON chat_contacts;
    DROP POLICY IF EXISTS tenant_isolation_chat_inboxes ON chat_inboxes;
END
$$;

DROP TABLE IF EXISTS chat_messages CASCADE;
DROP TABLE IF EXISTS chat_conversations CASCADE;
DROP TABLE IF EXISTS chat_contacts CASCADE;
DROP TABLE IF EXISTS chat_inboxes CASCADE;
