-- +goose Up

-- Contacts
CREATE TABLE IF NOT EXISTS contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    identifier TEXT NOT NULL,
    email TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(tenant_id, identifier)
);
ALTER TABLE contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY contacts_tenant_isolation_policy ON contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Inboxes
CREATE TABLE IF NOT EXISTS inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    enable_auto_assignment BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY inboxes_tenant_isolation_policy ON inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Channel Adapters
CREATE TABLE IF NOT EXISTS channel_adapters (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID REFERENCES inboxes(id),
    type TEXT NOT NULL,
    config JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE channel_adapters ENABLE ROW LEVEL SECURITY;
CREATE POLICY channel_adapters_tenant_isolation_policy ON channel_adapters FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Conversations
CREATE TABLE IF NOT EXISTS conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES inboxes(id),
    contact_id UUID NOT NULL REFERENCES contacts(id),
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY conversations_tenant_isolation_policy ON conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Messages
CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES conversations(id),
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY messages_tenant_isolation_policy ON messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- +goose Down
DROP TABLE IF EXISTS messages CASCADE;
DROP TABLE IF EXISTS conversations CASCADE;
DROP TABLE IF EXISTS channel_adapters CASCADE;
DROP TABLE IF EXISTS inboxes CASCADE;
DROP TABLE IF EXISTS contacts CASCADE;
