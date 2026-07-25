-- Migration 218: Native Omnichannel Chat

CREATE TABLE IF NOT EXISTS omni_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE omni_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_inboxes_tenant_policy ON omni_inboxes USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE TABLE IF NOT EXISTS omni_channel_adapters (
    id UUID PRIMARY KEY,
    inbox_id UUID NOT NULL,
    tenant_id UUID NOT NULL,
    provider_type TEXT NOT NULL,
    config JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE omni_channel_adapters ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_channel_adapters_tenant_policy ON omni_channel_adapters USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE TABLE IF NOT EXISTS omni_conversations (
    id UUID PRIMARY KEY,
    inbox_id UUID NOT NULL,
    tenant_id UUID NOT NULL,
    contact_id UUID,
    status TEXT NOT NULL,
    channel TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE omni_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_conversations_tenant_policy ON omni_conversations USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE TABLE IF NOT EXISTS omni_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL,
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    direction TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE omni_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_messages_tenant_policy ON omni_messages USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

CREATE TABLE IF NOT EXISTS omni_contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE omni_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_contacts_tenant_policy ON omni_contacts USING (tenant_id = current_setting('app.current_tenant_id')::UUID);
