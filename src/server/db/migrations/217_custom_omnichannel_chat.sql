-- +goose Up

CREATE TABLE IF NOT EXISTS chat_inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_chat_inboxes_tenant ON chat_inboxes(tenant_id);

CREATE TABLE IF NOT EXISTS chat_contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    email TEXT,
    phone_number TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_chat_contacts_tenant ON chat_contacts(tenant_id);

CREATE TABLE IF NOT EXISTS chat_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES chat_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_chat_conversations_tenant ON chat_conversations(tenant_id);
CREATE INDEX IF NOT EXISTS idx_chat_conversations_inbox ON chat_conversations(inbox_id);

CREATE TABLE IF NOT EXISTS chat_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL,
    content TEXT NOT NULL,
    is_draft BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_chat_messages_tenant ON chat_messages(tenant_id);
CREATE INDEX IF NOT EXISTS idx_chat_messages_conv ON chat_messages(conversation_id);

-- Apply RLS
DO $$
BEGIN
    IF to_regclass('chat_inboxes') IS NOT NULL THEN
        ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_chat_inboxes ON chat_inboxes;
        CREATE POLICY tenant_isolation_chat_inboxes ON chat_inboxes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_contacts') IS NOT NULL THEN
        ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_chat_contacts ON chat_contacts;
        CREATE POLICY tenant_isolation_chat_contacts ON chat_contacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_conversations') IS NOT NULL THEN
        ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_chat_conversations ON chat_conversations;
        CREATE POLICY tenant_isolation_chat_conversations ON chat_conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_messages') IS NOT NULL THEN
        ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_chat_messages ON chat_messages;
        CREATE POLICY tenant_isolation_chat_messages ON chat_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_chat_messages ON chat_messages;
DROP TABLE IF EXISTS chat_messages CASCADE;

DROP POLICY IF EXISTS tenant_isolation_chat_conversations ON chat_conversations;
DROP TABLE IF EXISTS chat_conversations CASCADE;

DROP POLICY IF EXISTS tenant_isolation_chat_contacts ON chat_contacts;
DROP TABLE IF EXISTS chat_contacts CASCADE;

DROP POLICY IF EXISTS tenant_isolation_chat_inboxes ON chat_inboxes;
DROP TABLE IF EXISTS chat_inboxes CASCADE;
