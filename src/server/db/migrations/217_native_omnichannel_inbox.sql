-- +goose Up
CREATE TABLE IF NOT EXISTS omnichannel_inboxes (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE omnichannel_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omnichannel_inboxes ON omnichannel_inboxes
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS omnichannel_channel_connections (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inboxes(id) ON DELETE CASCADE,
    provider_type TEXT NOT NULL,
    capabilities JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE omnichannel_channel_connections ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omnichannel_channel_connections ON omnichannel_channel_connections
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS omnichannel_contacts (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE omnichannel_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omnichannel_contacts ON omnichannel_contacts
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS omnichannel_contact_identities (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    contact_id UUID NOT NULL REFERENCES omnichannel_contacts(id) ON DELETE CASCADE,
    provider_type TEXT NOT NULL,
    provider_id TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_tenant_provider_identity UNIQUE (tenant_id, provider_type, provider_id)
);
ALTER TABLE omnichannel_contact_identities ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omnichannel_contact_identities ON omnichannel_contact_identities
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS omnichannel_conversations (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES omnichannel_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    priority INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE omnichannel_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omnichannel_conversations ON omnichannel_conversations
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS omnichannel_messages (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omnichannel_conversations(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    sender_id TEXT,
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE omnichannel_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_omnichannel_messages ON omnichannel_messages
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS omnichannel_messages CASCADE;
DROP TABLE IF EXISTS omnichannel_conversations CASCADE;
DROP TABLE IF EXISTS omnichannel_contact_identities CASCADE;
DROP TABLE IF EXISTS omnichannel_contacts CASCADE;
DROP TABLE IF EXISTS omnichannel_channel_connections CASCADE;
DROP TABLE IF EXISTS omnichannel_inboxes CASCADE;
