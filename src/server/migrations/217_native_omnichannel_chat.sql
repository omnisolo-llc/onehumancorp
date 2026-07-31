CREATE TABLE IF NOT EXISTS chat_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_inboxes_tenant_isolation_policy ON chat_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_channels (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
    channel_type TEXT NOT NULL,
    config JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_channels ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_channels_tenant_isolation_policy ON chat_channels FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone_number TEXT,
    custom_attributes JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_contacts_tenant_isolation_policy ON chat_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES chat_contacts(id) ON DELETE CASCADE,
    assignee_id UUID,
    status TEXT NOT NULL DEFAULT 'open',
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_conversations_tenant_isolation_policy ON chat_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL, -- e.g. 'contact', 'agent', 'bot'
    sender_id UUID,
    content TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_messages_tenant_isolation_policy ON chat_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- High-performance indexes for typical chat app query patterns
CREATE INDEX IF NOT EXISTS idx_chat_inboxes_tenant_id ON chat_inboxes(tenant_id);

CREATE INDEX IF NOT EXISTS idx_chat_channels_tenant_inbox ON chat_channels(tenant_id, inbox_id);

CREATE INDEX IF NOT EXISTS idx_chat_contacts_tenant_id ON chat_contacts(tenant_id);

CREATE INDEX IF NOT EXISTS idx_chat_conversations_tenant_inbox ON chat_conversations(tenant_id, inbox_id);
CREATE INDEX IF NOT EXISTS idx_chat_conversations_tenant_contact ON chat_conversations(tenant_id, contact_id);
CREATE INDEX IF NOT EXISTS idx_chat_conversations_tenant_status_updated ON chat_conversations(tenant_id, status, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_chat_messages_tenant_conversation_created ON chat_messages(tenant_id, conversation_id, created_at ASC);
