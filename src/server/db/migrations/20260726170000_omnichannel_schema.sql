-- Inboxes
CREATE TABLE omnichannel_inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_omnichannel_inboxes_tenant ON omnichannel_inboxes(tenant_id);

-- Channels
CREATE TABLE omnichannel_channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inboxes(id) ON DELETE CASCADE,
    provider_type TEXT NOT NULL,
    credentials JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_omnichannel_channels_tenant ON omnichannel_channels(tenant_id);

-- Contacts
CREATE TABLE omnichannel_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_omnichannel_contacts_tenant ON omnichannel_contacts(tenant_id);

-- Conversations
CREATE TABLE omnichannel_conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES omnichannel_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_omnichannel_conversations_tenant ON omnichannel_conversations(tenant_id);

-- Messages
CREATE TABLE omnichannel_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omnichannel_conversations(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL, -- e.g. contact, agent, bot
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_omnichannel_messages_tenant ON omnichannel_messages(tenant_id);
CREATE INDEX idx_omnichannel_messages_conversation ON omnichannel_messages(conversation_id);

-- Tenant row level security would typically go here
