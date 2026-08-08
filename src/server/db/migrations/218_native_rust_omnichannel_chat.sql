-- Create native rust omnichannel chat tables

-- inboxes
CREATE TABLE inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    channel_config JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY inboxes_tenant_policy ON inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));

-- contacts
CREATE TABLE chat_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    identifier TEXT,
    name TEXT NOT NULL DEFAULT '',
    email TEXT,
    phone_number TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_contacts_tenant_policy ON chat_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));

-- conversations
CREATE TABLE chat_conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    contact_id UUID REFERENCES chat_contacts(id) ON DELETE CASCADE,
    inbox_id UUID REFERENCES inboxes(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_conversations_tenant_policy ON chat_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));

-- messages
CREATE TABLE chat_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    conversation_id UUID REFERENCES chat_conversations(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL,
    sender_id TEXT,
    content TEXT,
    message_type TEXT NOT NULL DEFAULT 'incoming',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_messages_tenant_policy ON chat_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));
