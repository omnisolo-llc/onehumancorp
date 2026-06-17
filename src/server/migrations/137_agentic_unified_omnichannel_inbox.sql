-- +goose Up
CREATE TABLE IF NOT EXISTS unified_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    channel_provider TEXT NOT NULL,
    channel_identifier TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_unified_conversations_tenant_customer ON unified_conversations(tenant_id, customer_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_unified_conversations_unique_identifier ON unified_conversations(tenant_id, channel_provider, channel_identifier);
CREATE INDEX IF NOT EXISTS idx_unified_conversations_tenant_status ON unified_conversations(tenant_id, status);

DO $$
BEGIN
    IF to_regclass('unified_conversations') IS NOT NULL THEN
        ALTER TABLE unified_conversations ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_unified_conversations ON unified_conversations;
        CREATE POLICY tenant_isolation_unified_conversations ON unified_conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS unified_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES unified_conversations(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL,
    sender_id TEXT,
    content TEXT NOT NULL,
    intent_metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_unified_messages_conversation ON unified_messages(conversation_id, created_at ASC);

DO $$
BEGIN
    IF to_regclass('unified_messages') IS NOT NULL THEN
        ALTER TABLE unified_messages ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
        CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS unified_action_cards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES unified_conversations(id) ON DELETE CASCADE,
    message_id TEXT REFERENCES unified_messages(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    proposed_content TEXT,
    context_used JSONB,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_unified_action_cards_tenant_status ON unified_action_cards(tenant_id, status, created_at DESC);

DO $$
BEGIN
    IF to_regclass('unified_action_cards') IS NOT NULL THEN
        ALTER TABLE unified_action_cards ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_unified_action_cards ON unified_action_cards;
        CREATE POLICY tenant_isolation_unified_action_cards ON unified_action_cards USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_unified_action_cards ON unified_action_cards;
    DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
    DROP POLICY IF EXISTS tenant_isolation_unified_conversations ON unified_conversations;
END
$$;

DROP TABLE IF EXISTS unified_action_cards CASCADE;
DROP TABLE IF EXISTS unified_messages CASCADE;
DROP TABLE IF EXISTS unified_conversations CASCADE;
