-- Migration 218: Native Chat System

CREATE TABLE IF NOT EXISTS native_chat_inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    channel_type TEXT NOT NULL, -- 'web_widget', 'whatsapp', 'ig', etc.
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS native_chat_contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    avatar_url TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS native_chat_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES native_chat_inboxes(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES native_chat_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS native_chat_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES native_chat_conversations(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL, -- 'contact', 'agent', 'bot'
    sender_id TEXT, -- could be an agent ID or contact ID
    content TEXT NOT NULL,
    is_ai_draft BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Apply Row Level Security (RLS)
ALTER TABLE native_chat_inboxes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_native_chat_inboxes ON native_chat_inboxes;
CREATE POLICY tenant_isolation_native_chat_inboxes ON native_chat_inboxes
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE native_chat_contacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_native_chat_contacts ON native_chat_contacts;
CREATE POLICY tenant_isolation_native_chat_contacts ON native_chat_contacts
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE native_chat_conversations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_native_chat_conversations ON native_chat_conversations;
CREATE POLICY tenant_isolation_native_chat_conversations ON native_chat_conversations
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE native_chat_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_native_chat_messages ON native_chat_messages;
CREATE POLICY tenant_isolation_native_chat_messages ON native_chat_messages
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
