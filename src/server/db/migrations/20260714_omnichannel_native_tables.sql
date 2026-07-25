-- Omnichannel Native Rust Tables
-- Enforcing Multi-Tenant Row Level Security (RLS)

CREATE TABLE IF NOT EXISTS omnichannel_native_inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

ALTER TABLE omnichannel_native_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON omnichannel_native_inboxes
    USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE TABLE IF NOT EXISTS omnichannel_native_channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inbox_id UUID NOT NULL REFERENCES omnichannel_native_inboxes(id) ON DELETE CASCADE,
    provider_type TEXT NOT NULL,
    credentials JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS omnichannel_native_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

ALTER TABLE omnichannel_native_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON omnichannel_native_contacts
    USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE TABLE IF NOT EXISTS omnichannel_native_conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omnichannel_native_inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES omnichannel_native_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

ALTER TABLE omnichannel_native_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON omnichannel_native_conversations
    USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE TABLE IF NOT EXISTS omnichannel_native_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id UUID NOT NULL REFERENCES omnichannel_native_conversations(id) ON DELETE CASCADE,
    sender_id UUID,
    sender_type TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT DEFAULT 'sent',
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);
