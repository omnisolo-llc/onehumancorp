CREATE TABLE IF NOT EXISTS omnichat_inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

ALTER TABLE omnichat_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichat_inboxes_tenant_isolation ON omnichat_inboxes
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichat_channel_adapters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inbox_id UUID NOT NULL REFERENCES omnichat_inboxes(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL,
    channel_type TEXT NOT NULL,
    config JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

ALTER TABLE omnichat_channel_adapters ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichat_channel_adapters_tenant_isolation ON omnichat_channel_adapters
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichat_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

ALTER TABLE omnichat_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichat_contacts_tenant_isolation ON omnichat_contacts
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichat_conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inbox_id UUID NOT NULL REFERENCES omnichat_inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES omnichat_contacts(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

ALTER TABLE omnichat_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichat_conversations_tenant_isolation ON omnichat_conversations
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichat_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id UUID NOT NULL REFERENCES omnichat_conversations(id) ON DELETE CASCADE,
    contact_id UUID REFERENCES omnichat_contacts(id) ON DELETE SET NULL,
    tenant_id UUID NOT NULL,
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'sent',
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

ALTER TABLE omnichat_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichat_messages_tenant_isolation ON omnichat_messages
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
