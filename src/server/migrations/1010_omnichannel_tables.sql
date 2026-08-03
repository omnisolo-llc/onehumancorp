CREATE TABLE IF NOT EXISTS omni_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    auto_assignment_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_omni_inboxes_tenant_id ON omni_inboxes(tenant_id);

ALTER TABLE omni_inboxes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS omni_inboxes_tenant_isolation_policy ON omni_inboxes;
CREATE POLICY omni_inboxes_tenant_isolation_policy ON omni_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);


CREATE TABLE IF NOT EXISTS omni_contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    identifier TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_omni_contacts_tenant_id ON omni_contacts(tenant_id);

ALTER TABLE omni_contacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS omni_contacts_tenant_isolation_policy ON omni_contacts;
CREATE POLICY omni_contacts_tenant_isolation_policy ON omni_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);


CREATE TABLE IF NOT EXISTS omni_conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omni_inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES omni_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    assignee_id UUID,
    unread_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_omni_conversations_tenant_id ON omni_conversations(tenant_id);
CREATE INDEX IF NOT EXISTS idx_omni_conversations_inbox_id ON omni_conversations(inbox_id);
CREATE INDEX IF NOT EXISTS idx_omni_conversations_contact_id ON omni_conversations(contact_id);

ALTER TABLE omni_conversations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS omni_conversations_tenant_isolation_policy ON omni_conversations;
CREATE POLICY omni_conversations_tenant_isolation_policy ON omni_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);


CREATE TABLE IF NOT EXISTS omni_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omni_conversations(id) ON DELETE CASCADE,
    inbox_id UUID NOT NULL REFERENCES omni_inboxes(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL,
    sender_id UUID,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL DEFAULT 'text',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_omni_messages_tenant_id ON omni_messages(tenant_id);
CREATE INDEX IF NOT EXISTS idx_omni_messages_conversation_id ON omni_messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_omni_messages_inbox_id ON omni_messages(inbox_id);

ALTER TABLE omni_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS omni_messages_tenant_isolation_policy ON omni_messages;
CREATE POLICY omni_messages_tenant_isolation_policy ON omni_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
