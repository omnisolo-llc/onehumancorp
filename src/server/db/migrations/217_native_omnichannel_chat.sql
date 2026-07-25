-- +goose Up

CREATE TABLE IF NOT EXISTS native_chat_inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE native_chat_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_native_chat_inboxes ON native_chat_inboxes
    FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS native_chat_channels (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES native_chat_inboxes(id) ON DELETE CASCADE,
    channel_type TEXT NOT NULL,
    config JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE native_chat_channels ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_native_chat_channels ON native_chat_channels
    FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS native_chat_contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    avatar_url TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE native_chat_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_native_chat_contacts ON native_chat_contacts
    FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS native_chat_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES native_chat_inboxes(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES native_chat_contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE native_chat_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_native_chat_conversations ON native_chat_conversations
    FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS native_chat_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES native_chat_conversations(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE native_chat_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_native_chat_messages ON native_chat_messages
    FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_native_chat_messages ON native_chat_messages;
DROP TABLE IF EXISTS native_chat_messages CASCADE;

DROP POLICY IF EXISTS tenant_isolation_native_chat_conversations ON native_chat_conversations;
DROP TABLE IF EXISTS native_chat_conversations CASCADE;

DROP POLICY IF EXISTS tenant_isolation_native_chat_contacts ON native_chat_contacts;
DROP TABLE IF EXISTS native_chat_contacts CASCADE;

DROP POLICY IF EXISTS tenant_isolation_native_chat_channels ON native_chat_channels;
DROP TABLE IF EXISTS native_chat_channels CASCADE;

DROP POLICY IF EXISTS tenant_isolation_native_chat_inboxes ON native_chat_inboxes;
DROP TABLE IF EXISTS native_chat_inboxes CASCADE;
