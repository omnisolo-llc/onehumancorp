CREATE TABLE inboxes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    channel_type VARCHAR(255) NOT NULL,
    email_address VARCHAR(255),
    greeting_enabled BOOLEAN DEFAULT FALSE,
    greeting_message TEXT,
    working_hours_enabled BOOLEAN DEFAULT FALSE,
    csat_survey_enabled BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_inboxes ON inboxes USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE TABLE conversations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    inbox_id UUID NOT NULL REFERENCES inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'open',
    priority INTEGER DEFAULT 0,
    agent_last_seen_at TIMESTAMPTZ,
    contact_last_seen_at TIMESTAMPTZ,
    first_reply_created_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_conversations ON conversations USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE TABLE chat_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    inbox_id UUID NOT NULL REFERENCES inboxes(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    message_type VARCHAR(50) NOT NULL,
    content_type VARCHAR(50) NOT NULL DEFAULT 'text',
    status VARCHAR(50) NOT NULL DEFAULT 'sent',
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    sender_type VARCHAR(50) NOT NULL,
    sender_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_chat_messages ON chat_messages USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE INDEX idx_chat_messages_conversation_id ON chat_messages(conversation_id);
CREATE INDEX idx_chat_messages_tenant_created_at ON chat_messages(tenant_id, created_at);
CREATE INDEX idx_conversations_tenant_status ON conversations(tenant_id, status);
CREATE INDEX idx_conversations_inbox_id ON conversations(inbox_id);
