-- +goose Up
CREATE TABLE IF NOT EXISTS chat_inbox (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_contact (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_conversation (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES chat_inbox(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES chat_contact(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS chat_message (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES chat_conversation(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    sender_id TEXT,
    status TEXT NOT NULL DEFAULT 'sent',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('chat_inbox') IS NOT NULL THEN
        ALTER TABLE chat_inbox ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_chat_inbox ON chat_inbox;
        CREATE POLICY tenant_isolation_chat_inbox ON chat_inbox USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_contact') IS NOT NULL THEN
        ALTER TABLE chat_contact ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_chat_contact ON chat_contact;
        CREATE POLICY tenant_isolation_chat_contact ON chat_contact USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_conversation') IS NOT NULL THEN
        ALTER TABLE chat_conversation ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_chat_conversation ON chat_conversation;
        CREATE POLICY tenant_isolation_chat_conversation ON chat_conversation USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('chat_message') IS NOT NULL THEN
        ALTER TABLE chat_message ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_chat_message ON chat_message;
        CREATE POLICY tenant_isolation_chat_message ON chat_message USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_chat_inbox ON chat_inbox;
    DROP POLICY IF EXISTS tenant_isolation_chat_contact ON chat_contact;
    DROP POLICY IF EXISTS tenant_isolation_chat_conversation ON chat_conversation;
    DROP POLICY IF EXISTS tenant_isolation_chat_message ON chat_message;
END
$$;

DROP TABLE IF EXISTS chat_message CASCADE;
DROP TABLE IF EXISTS chat_conversation CASCADE;
DROP TABLE IF EXISTS chat_contact CASCADE;
DROP TABLE IF EXISTS chat_inbox CASCADE;
