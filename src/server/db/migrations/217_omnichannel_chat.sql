-- +goose Up

CREATE TABLE IF NOT EXISTS chat_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    settings JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_chat_inboxes ON chat_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone_number TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_chat_contacts ON chat_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_contact_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    contact_id UUID NOT NULL REFERENCES chat_contacts(id),
    inbox_id UUID NOT NULL REFERENCES chat_inboxes(id),
    source_id TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE chat_contact_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_chat_contact_inboxes ON chat_contact_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES chat_inboxes(id),
    contact_id UUID NOT NULL REFERENCES chat_contacts(id),
    status TEXT NOT NULL,
    assignee_id UUID,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_chat_conversations ON chat_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES chat_conversations(id),
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    sender_id UUID,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_chat_messages ON chat_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)::uuid) WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);


-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_chat_messages ON chat_messages;
DROP POLICY IF EXISTS tenant_isolation_chat_conversations ON chat_conversations;
DROP POLICY IF EXISTS tenant_isolation_chat_contact_inboxes ON chat_contact_inboxes;
DROP POLICY IF EXISTS tenant_isolation_chat_contacts ON chat_contacts;
DROP POLICY IF EXISTS tenant_isolation_chat_inboxes ON chat_inboxes;

DROP TABLE IF EXISTS chat_messages CASCADE;
DROP TABLE IF EXISTS chat_conversations CASCADE;
DROP TABLE IF EXISTS chat_contact_inboxes CASCADE;
DROP TABLE IF EXISTS chat_contacts CASCADE;
DROP TABLE IF EXISTS chat_inboxes CASCADE;
