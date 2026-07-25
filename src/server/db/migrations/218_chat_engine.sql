-- +goose Up
-- Migration 218: Rust Native Chat & Omnichannel Routing Engine

CREATE TABLE IF NOT EXISTS ohc_chat_channels (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    provider TEXT NOT NULL,
    name TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ohc_chat_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    channel_id TEXT NOT NULL REFERENCES ohc_chat_channels(id) ON DELETE CASCADE,
    assignee_id TEXT,
    customer_id TEXT,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ohc_chat_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES ohc_chat_conversations(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL,
    content TEXT NOT NULL,
    ai_draft_status TEXT NOT NULL DEFAULT 'none',
    draft_content TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('ohc_chat_channels') IS NOT NULL THEN
        ALTER TABLE ohc_chat_channels ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_ohc_chat_channels ON ohc_chat_channels USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('ohc_chat_conversations') IS NOT NULL THEN
        ALTER TABLE ohc_chat_conversations ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_ohc_chat_conversations ON ohc_chat_conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('ohc_chat_messages') IS NOT NULL THEN
        ALTER TABLE ohc_chat_messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_ohc_chat_messages ON ohc_chat_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_ohc_chat_channels ON ohc_chat_channels;
    DROP POLICY IF EXISTS tenant_isolation_ohc_chat_conversations ON ohc_chat_conversations;
    DROP POLICY IF EXISTS tenant_isolation_ohc_chat_messages ON ohc_chat_messages;
END
$$;

DROP TABLE IF EXISTS ohc_chat_messages CASCADE;
DROP TABLE IF EXISTS ohc_chat_conversations CASCADE;
DROP TABLE IF EXISTS ohc_chat_channels CASCADE;
