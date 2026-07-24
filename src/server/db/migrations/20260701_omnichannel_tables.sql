CREATE TABLE IF NOT EXISTS omnichannel_contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone_number TEXT,
    custom_attributes JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_contacts_tenant_isolation_policy ON omnichannel_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    channel_config JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_inboxes_tenant_isolation_policy ON omnichannel_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES omnichannel_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    assignee_id UUID,
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_conversations_tenant_isolation_policy ON omnichannel_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omnichannel_conversations(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL,
    sender_type TEXT NOT NULL,
    message_type TEXT NOT NULL,
    content TEXT NOT NULL,
    external_source_ids JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_messages_tenant_isolation_policy ON omnichannel_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
