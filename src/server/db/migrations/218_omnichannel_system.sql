-- Omnichannel System Tables

CREATE TABLE IF NOT EXISTS omnichannel_tenants (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE omnichannel_tenants ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_tenants ON omnichannel_tenants;
CREATE POLICY tenant_isolation_omnichannel_tenants ON omnichannel_tenants
USING (id = current_setting('app.current_tenant', true))
WITH CHECK (id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS omnichannel_inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE omnichannel_inboxes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_inboxes ON omnichannel_inboxes;
CREATE POLICY tenant_isolation_omnichannel_inboxes ON omnichannel_inboxes
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS omnichannel_channels (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE omnichannel_channels ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_channels ON omnichannel_channels;
CREATE POLICY tenant_isolation_omnichannel_channels ON omnichannel_channels
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS omnichannel_contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE omnichannel_contacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_contacts ON omnichannel_contacts;
CREATE POLICY tenant_isolation_omnichannel_contacts ON omnichannel_contacts
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS omnichannel_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL,
    contact_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE omnichannel_conversations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_conversations ON omnichannel_conversations;
CREATE POLICY tenant_isolation_omnichannel_conversations ON omnichannel_conversations
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS omnichannel_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'delivered',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE omnichannel_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omnichannel_messages ON omnichannel_messages;
CREATE POLICY tenant_isolation_omnichannel_messages ON omnichannel_messages
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
