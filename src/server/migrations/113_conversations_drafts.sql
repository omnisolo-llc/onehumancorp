-- +goose Up
-- Migration 113: Add conversations and draft_replies tables

CREATE TABLE IF NOT EXISTS conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_conversations_tenant ON conversations(tenant_id);

CREATE TABLE IF NOT EXISTS draft_replies (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    message_id TEXT NOT NULL REFERENCES inbox_messages(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(message_id)
);

CREATE INDEX IF NOT EXISTS idx_draft_replies_tenant ON draft_replies(tenant_id);

ALTER TABLE inbox_messages ADD COLUMN IF NOT EXISTS conversation_id TEXT REFERENCES conversations(id) ON DELETE SET NULL;
ALTER TABLE inbox_messages ADD COLUMN IF NOT EXISTS direction TEXT DEFAULT 'inbound';
ALTER TABLE inbox_messages ADD COLUMN IF NOT EXISTS channel TEXT;

DO $$
BEGIN
    IF to_regclass('conversations') IS NOT NULL THEN
        ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_conversations ON conversations;
        CREATE POLICY tenant_isolation_conversations ON conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('draft_replies') IS NOT NULL THEN
        ALTER TABLE draft_replies ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_draft_replies ON draft_replies;
        CREATE POLICY tenant_isolation_draft_replies ON draft_replies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_conversations ON conversations;
    DROP POLICY IF EXISTS tenant_isolation_draft_replies ON draft_replies;
END
$$;

ALTER TABLE inbox_messages DROP COLUMN IF EXISTS conversation_id;
ALTER TABLE inbox_messages DROP COLUMN IF EXISTS direction;
ALTER TABLE inbox_messages DROP COLUMN IF EXISTS channel;

DROP TABLE IF EXISTS draft_replies CASCADE;
DROP TABLE IF EXISTS conversations CASCADE;
