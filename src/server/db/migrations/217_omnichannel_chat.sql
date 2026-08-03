CREATE TABLE IF NOT EXISTS chat_inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_inboxes_tenant_isolation_policy ON chat_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone_number TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_contacts_tenant_isolation_policy ON chat_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES chat_inboxes(id),
    contact_id UUID NOT NULL REFERENCES chat_contacts(id),
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_conversations_tenant_isolation_policy ON chat_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE INDEX IF NOT EXISTS idx_chat_conversations_tenant_inbox ON chat_conversations(tenant_id, inbox_id);

CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES chat_conversations(id),
    inbox_id UUID NOT NULL REFERENCES chat_inboxes(id),
    sender_type TEXT NOT NULL,
    sender_id UUID,
    content_type TEXT NOT NULL DEFAULT 'text',
    content TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_messages_tenant_isolation_policy ON chat_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE INDEX IF NOT EXISTS idx_chat_messages_tenant_conversation ON chat_messages(tenant_id, conversation_id);
