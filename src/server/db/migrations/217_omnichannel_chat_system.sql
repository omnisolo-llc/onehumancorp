-- +goose Up
-- Migration 217: Omnichannel Chat System Core Tables

CREATE TABLE IF NOT EXISTS inboxes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS channels (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES inboxes(id) ON DELETE CASCADE,
    provider_type TEXT NOT NULL,
    credentials JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS contacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    phone_number TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES inboxes(id) ON DELETE CASCADE,
    contact_id TEXT NOT NULL REFERENCES contacts(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    is_private_note BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('inboxes') IS NOT NULL THEN
        ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_inboxes ON inboxes;
        CREATE POLICY tenant_isolation_inboxes ON inboxes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('channels') IS NOT NULL THEN
        ALTER TABLE channels ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_channels ON channels;
        CREATE POLICY tenant_isolation_channels ON channels USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('contacts') IS NOT NULL THEN
        ALTER TABLE contacts ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_contacts ON contacts;
        CREATE POLICY tenant_isolation_contacts ON contacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('conversations') IS NOT NULL THEN
        ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_conversations ON conversations;
        CREATE POLICY tenant_isolation_conversations ON conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('messages') IS NOT NULL THEN
        ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_messages ON messages;
        CREATE POLICY tenant_isolation_messages ON messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_messages ON messages;
    DROP POLICY IF EXISTS tenant_isolation_conversations ON conversations;
    DROP POLICY IF EXISTS tenant_isolation_contacts ON contacts;
    DROP POLICY IF EXISTS tenant_isolation_channels ON channels;
    DROP POLICY IF EXISTS tenant_isolation_inboxes ON inboxes;
END
$$;

DROP TABLE IF EXISTS messages CASCADE;
DROP TABLE IF EXISTS conversations CASCADE;
DROP TABLE IF EXISTS contacts CASCADE;
DROP TABLE IF EXISTS channels CASCADE;
DROP TABLE IF EXISTS inboxes CASCADE;
