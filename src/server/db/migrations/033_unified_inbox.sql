-- +goose Up
-- Migration 033: Add unified inbox tables

CREATE TABLE IF NOT EXISTS conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_conversations_tenant_customer ON conversations(tenant_id, customer_id);
CREATE INDEX IF NOT EXISTS idx_conversations_tenant_status ON conversations(tenant_id, status);

DO $$
BEGIN
    IF to_regclass('conversations') IS NOT NULL THEN
        ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_conversations ON conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    channel TEXT NOT NULL,
    direction TEXT NOT NULL,
    content TEXT NOT NULL,
    original_content TEXT,
    translated_from_language TEXT,
    sender_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(tenant_id, conversation_id);
CREATE INDEX IF NOT EXISTS idx_messages_tenant_created_at ON messages(tenant_id, created_at DESC);

DO $$
BEGIN
    IF to_regclass('messages') IS NOT NULL THEN
        ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_messages ON messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;


CREATE TABLE IF NOT EXISTS draft_replies (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    message_id TEXT NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_draft_replies_message ON draft_replies(tenant_id, message_id);

DO $$
BEGIN
    IF to_regclass('draft_replies') IS NOT NULL THEN
        ALTER TABLE draft_replies ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_draft_replies ON draft_replies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

INSERT INTO conversations (id, tenant_id, status, created_at)
SELECT id, tenant_id, status, created_at FROM inbox_messages;

INSERT INTO messages (id, tenant_id, conversation_id, channel, direction, content, original_content, translated_from_language, sender_id, created_at)
SELECT id, tenant_id, id, source, 'inbound', content, original_content, translated_from_language, sender_id, created_at FROM inbox_messages;

INSERT INTO draft_replies (id, tenant_id, message_id, content, status, created_at)
SELECT id, tenant_id, id, draft_reply, 'pending', created_at FROM inbox_messages;


-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_draft_replies ON draft_replies;
    DROP POLICY IF EXISTS tenant_isolation_messages ON messages;
    DROP POLICY IF EXISTS tenant_isolation_conversations ON conversations;
END
$$;

DROP TABLE IF EXISTS draft_replies CASCADE;
DROP TABLE IF EXISTS messages CASCADE;
DROP TABLE IF EXISTS conversations CASCADE;
