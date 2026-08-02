-- Migration 1009: Native Omnichannel Chat Architecture
-- Replaces external Chatwoot dependency with native Rust tables

CREATE TABLE IF NOT EXISTS chat_inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL, -- e.g., 'web_widget', 'email', 'sms', 'instagram'
    config JSONB DEFAULT '{}'::jsonb,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    email TEXT,
    phone_number TEXT,
    avatar_url TEXT,
    custom_attributes JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES chat_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open', -- 'open', 'resolved', 'pending', 'snoozed'
    assignee_id TEXT, -- User ID (can be NULL if unassigned)
    unread_count INTEGER DEFAULT 0,
    custom_attributes JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL, -- 'contact', 'agent', 'bot', 'system'
    sender_id TEXT, -- ID of the sender (Contact ID, User ID, or Bot ID)
    content TEXT NOT NULL,
    content_type TEXT DEFAULT 'text', -- 'text', 'html', 'image', 'file', 'template'
    status TEXT DEFAULT 'sent', -- 'sent', 'delivered', 'read', 'failed'
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_chat_inboxes_tenant ON chat_inboxes(tenant_id);
CREATE INDEX IF NOT EXISTS idx_chat_contacts_tenant ON chat_contacts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_chat_conversations_tenant_inbox ON chat_conversations(tenant_id, inbox_id);
CREATE INDEX IF NOT EXISTS idx_chat_conversations_tenant_contact ON chat_conversations(tenant_id, contact_id);
CREATE INDEX IF NOT EXISTS idx_chat_messages_tenant_conversation ON chat_messages(tenant_id, conversation_id);

-- Enforce RLS
ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;
ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;

-- Add RLS Policies
CREATE POLICY tenant_isolation_chat_inboxes ON chat_inboxes
    USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_chat_contacts ON chat_contacts
    USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_chat_conversations ON chat_conversations
    USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_chat_messages ON chat_messages
    USING (tenant_id = current_setting('app.current_tenant', true));
