CREATE TABLE IF NOT EXISTS chat_contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    account_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone_number TEXT,
    avatar_url TEXT,
    identifier TEXT,
    additional_attributes JSONB,
    custom_attributes JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_contacts_tenant_isolation_policy ON chat_contacts
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);


CREATE TABLE IF NOT EXISTS chat_conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    account_id UUID NOT NULL,
    inbox_id UUID NOT NULL,
    contact_id UUID NOT NULL REFERENCES chat_contacts(id),
    assignee_id UUID,
    status TEXT NOT NULL DEFAULT 'open',
    additional_attributes JSONB,
    custom_attributes JSONB,
    snoozed_until TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_conversations_tenant_isolation_policy ON chat_conversations
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);


CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    account_id UUID NOT NULL,
    inbox_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES chat_conversations(id),
    message_type TEXT NOT NULL,
    content TEXT,
    status TEXT NOT NULL,
    sender_id UUID,
    sender_type TEXT,
    source_id TEXT,
    additional_attributes JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_messages_tenant_isolation_policy ON chat_messages
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
