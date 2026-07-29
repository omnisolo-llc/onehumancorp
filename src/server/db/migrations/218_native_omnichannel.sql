-- +goose Up
-- Migration 218: Native Omnichannel Chat System

CREATE TABLE IF NOT EXISTS omnichannel_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    enable_auto_assignment BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_channels (
    id UUID PRIMARY KEY,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inboxes(id) ON DELETE CASCADE,
    provider_type TEXT NOT NULL,
    credentials JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    phone_number TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_conversations (
    id UUID PRIMARY KEY,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES omnichannel_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_messages (
    id UUID PRIMARY KEY,
    conversation_id UUID NOT NULL REFERENCES omnichannel_conversations(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- RLS Policies
ALTER TABLE omnichannel_inboxes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_inboxes ON omnichannel_inboxes;
CREATE POLICY tenant_isolation_omnichannel_inboxes ON omnichannel_inboxes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE omnichannel_contacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_contacts ON omnichannel_contacts;
CREATE POLICY tenant_isolation_omnichannel_contacts ON omnichannel_contacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE omnichannel_channels ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_channels ON omnichannel_channels;
CREATE POLICY tenant_isolation_omnichannel_channels ON omnichannel_channels USING (
    EXISTS (
        SELECT 1 FROM omnichannel_inboxes
        WHERE omnichannel_inboxes.id = omnichannel_channels.inbox_id
        AND omnichannel_inboxes.tenant_id::text = current_setting('app.current_tenant', true)
    )
) WITH CHECK (
    EXISTS (
        SELECT 1 FROM omnichannel_inboxes
        WHERE omnichannel_inboxes.id = omnichannel_channels.inbox_id
        AND omnichannel_inboxes.tenant_id::text = current_setting('app.current_tenant', true)
    )
);

ALTER TABLE omnichannel_conversations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_conversations ON omnichannel_conversations;
CREATE POLICY tenant_isolation_omnichannel_conversations ON omnichannel_conversations USING (
    EXISTS (
        SELECT 1 FROM omnichannel_inboxes
        WHERE omnichannel_inboxes.id = omnichannel_conversations.inbox_id
        AND omnichannel_inboxes.tenant_id::text = current_setting('app.current_tenant', true)
    )
) WITH CHECK (
    EXISTS (
        SELECT 1 FROM omnichannel_inboxes
        WHERE omnichannel_inboxes.id = omnichannel_conversations.inbox_id
        AND omnichannel_inboxes.tenant_id::text = current_setting('app.current_tenant', true)
    )
);

ALTER TABLE omnichannel_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_messages ON omnichannel_messages;
CREATE POLICY tenant_isolation_omnichannel_messages ON omnichannel_messages USING (
    EXISTS (
        SELECT 1 FROM omnichannel_conversations
        JOIN omnichannel_inboxes ON omnichannel_conversations.inbox_id = omnichannel_inboxes.id
        WHERE omnichannel_conversations.id = omnichannel_messages.conversation_id
        AND omnichannel_inboxes.tenant_id::text = current_setting('app.current_tenant', true)
    )
) WITH CHECK (
    EXISTS (
        SELECT 1 FROM omnichannel_conversations
        JOIN omnichannel_inboxes ON omnichannel_conversations.inbox_id = omnichannel_inboxes.id
        WHERE omnichannel_conversations.id = omnichannel_messages.conversation_id
        AND omnichannel_inboxes.tenant_id::text = current_setting('app.current_tenant', true)
    )
);

-- +goose Down
DROP TABLE IF EXISTS omnichannel_messages CASCADE;
DROP TABLE IF EXISTS omnichannel_conversations CASCADE;
DROP TABLE IF EXISTS omnichannel_contacts CASCADE;
DROP TABLE IF EXISTS omnichannel_channels CASCADE;
DROP TABLE IF EXISTS omnichannel_inboxes CASCADE;
