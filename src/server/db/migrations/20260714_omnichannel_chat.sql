CREATE TABLE IF NOT EXISTS chat_contact (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_contact ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_contact_tenant_isolation_policy ON chat_contact FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_inbox (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_inbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_inbox_tenant_isolation_policy ON chat_inbox FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_conversation (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES chat_inbox(id),
    contact_id UUID NOT NULL REFERENCES chat_contact(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_conversation ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_conversation_tenant_isolation_policy ON chat_conversation FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_message (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES chat_conversation(id),
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    sender_id UUID,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_message ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_message_tenant_isolation_policy ON chat_message FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Add unique constraint to prevent race conditions during concurrent webhook requests
CREATE UNIQUE INDEX chat_conversation_tenant_inbox_contact_open_idx
ON chat_conversation (tenant_id, inbox_id, contact_id)
WHERE status = 'open';
