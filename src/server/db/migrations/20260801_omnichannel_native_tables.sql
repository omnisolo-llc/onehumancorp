CREATE TABLE IF NOT EXISTS omni_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omni_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_inboxes_tenant_isolation_policy ON omni_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omni_conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omni_inboxes(id),
    contact_id UUID NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    snoozed_until TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omni_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_conversations_tenant_isolation_policy ON omni_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omni_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omni_conversations(id),
    content TEXT NOT NULL,
    message_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'sent',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omni_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_messages_tenant_isolation_policy ON omni_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
