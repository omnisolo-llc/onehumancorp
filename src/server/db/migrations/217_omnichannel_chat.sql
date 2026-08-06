-- +goose Up
CREATE TABLE IF NOT EXISTS omni_inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    channel_type TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE omni_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omni_inboxes ON omni_inboxes
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS omni_contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE omni_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omni_contacts ON omni_contacts
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS omni_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES omni_inboxes(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES omni_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    last_activity_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE omni_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omni_conversations ON omni_conversations
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS omni_chat_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES omni_conversations(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL,
    sender_id TEXT,
    content TEXT,
    message_type TEXT NOT NULL DEFAULT 'incoming',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE omni_chat_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omni_chat_messages ON omni_chat_messages
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_omni_chat_messages ON omni_chat_messages;
DROP TABLE IF EXISTS omni_chat_messages CASCADE;

DROP POLICY IF EXISTS tenant_isolation_omni_conversations ON omni_conversations;
DROP TABLE IF EXISTS omni_conversations CASCADE;

DROP POLICY IF EXISTS tenant_isolation_omni_contacts ON omni_contacts;
DROP TABLE IF EXISTS omni_contacts CASCADE;

DROP POLICY IF EXISTS tenant_isolation_omni_inboxes ON omni_inboxes;
DROP TABLE IF EXISTS omni_inboxes CASCADE;
