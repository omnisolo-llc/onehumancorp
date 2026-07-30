-- Create chat_inboxes table
CREATE TABLE chat_inboxes (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    channel_type VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS
ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_inboxes ON chat_inboxes
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

-- Create chat_contacts table
CREATE TABLE chat_contacts (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR,
    phone_number VARCHAR,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_contacts ON chat_contacts
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

-- Create chat_conversations table
CREATE TABLE chat_conversations (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    inbox_id UUID NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES chat_contacts(id) ON DELETE CASCADE,
    status VARCHAR NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_conversations ON chat_conversations
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

-- Create chat_messages table
CREATE TABLE chat_messages (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    conversation_id UUID NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    message_type VARCHAR NOT NULL,
    content_attributes JSONB,
    external_source_ids JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_messages ON chat_messages
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));
