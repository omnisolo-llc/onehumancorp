CREATE TABLE IF NOT EXISTS omnichannel_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_inboxes_tenant_isolation_policy ON omnichannel_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    contact_id UUID NOT NULL REFERENCES customer_profile(id),
    inbox_id UUID NOT NULL REFERENCES omnichannel_inboxes(id),
    status TEXT NOT NULL DEFAULT 'open',
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_conversations_tenant_isolation_policy ON omnichannel_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omnichannel_conversations(id),
    content TEXT NOT NULL,
    message_type TEXT NOT NULL,
    sender_id UUID NOT NULL,
    sender_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'sent',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_messages_tenant_isolation_policy ON omnichannel_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
