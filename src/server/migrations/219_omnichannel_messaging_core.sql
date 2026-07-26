CREATE TABLE IF NOT EXISTS inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    channel_config JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS inboxes_tenant_id_idx ON inboxes (tenant_id);

DO $$
BEGIN
    IF to_regclass('inboxes') IS NOT NULL THEN
        ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;
        CREATE POLICY inboxes_tenant_isolation_policy ON inboxes FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    identifier TEXT NOT NULL,
    attributes JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS contacts_tenant_id_idx ON contacts (tenant_id);

DO $$
BEGIN
    IF to_regclass('contacts') IS NOT NULL THEN
        ALTER TABLE contacts ENABLE ROW LEVEL SECURITY;
        CREATE POLICY contacts_tenant_isolation_policy ON contacts FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES inboxes(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS conversations_tenant_id_idx ON conversations (tenant_id);
CREATE INDEX IF NOT EXISTS conversations_inbox_id_idx ON conversations (inbox_id);
CREATE INDEX IF NOT EXISTS conversations_contact_id_idx ON conversations (contact_id);

DO $$
BEGIN
    IF to_regclass('conversations') IS NOT NULL THEN
        ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
        CREATE POLICY conversations_tenant_isolation_policy ON conversations FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    sender_id TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    content TEXT NOT NULL,
    message_type TEXT NOT NULL,
    additional_attributes JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS messages_tenant_id_idx ON messages (tenant_id);
CREATE INDEX IF NOT EXISTS messages_conversation_id_idx ON messages (conversation_id);

DO $$
BEGIN
    IF to_regclass('messages') IS NOT NULL THEN
        ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY messages_tenant_isolation_policy ON messages FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
