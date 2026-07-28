-- Inboxes
CREATE TABLE chat_inboxes (
    id SERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    enable_auto_assignment BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenant_isolation_chat_inboxes" ON chat_inboxes
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Contacts
CREATE TABLE chat_contacts (
    id SERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone_number VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenant_isolation_chat_contacts" ON chat_contacts
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Conversations
CREATE TABLE chat_conversations (
    id SERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id INTEGER NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
    contact_id INTEGER NOT NULL REFERENCES chat_contacts(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenant_isolation_chat_conversations" ON chat_conversations
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Messages
CREATE TABLE chat_messages (
    id SERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id INTEGER NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    message_type INTEGER NOT NULL, -- 0 for incoming, 1 for outgoing, etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenant_isolation_chat_messages" ON chat_messages
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Channel Whatsapp
CREATE TABLE chat_channel_whatsapp (
    id SERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL,
    phone_number VARCHAR(255) NOT NULL UNIQUE,
    provider VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_channel_whatsapp ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenant_isolation_chat_channel_whatsapp" ON chat_channel_whatsapp
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Channel WebWidget
CREATE TABLE chat_channel_web_widget (
    id SERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL,
    website_url VARCHAR(255) NOT NULL,
    website_token VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE chat_channel_web_widget ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenant_isolation_chat_channel_web_widget" ON chat_channel_web_widget
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
