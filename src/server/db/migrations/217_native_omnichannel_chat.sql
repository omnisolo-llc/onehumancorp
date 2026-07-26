CREATE TABLE IF NOT EXISTS chat_inbox (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    settings JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_inbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_inbox_tenant_isolation_policy ON chat_inbox FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_contact (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone_number TEXT,
    custom_attributes JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_contact ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_contact_tenant_isolation_policy ON chat_contact FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_conversation (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES chat_inbox(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES chat_contact(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    priority INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_conversation ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_conversation_tenant_isolation_policy ON chat_conversation FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_message (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES chat_conversation(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL, -- 'contact', 'agent', 'bot', 'system'
    sender_id UUID, -- NULL if system
    content TEXT NOT NULL,
    content_type TEXT NOT NULL DEFAULT 'text',
    status TEXT NOT NULL DEFAULT 'sent', -- 'draft', 'sent', 'delivered', 'read', 'failed'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_message ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_message_tenant_isolation_policy ON chat_message FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
