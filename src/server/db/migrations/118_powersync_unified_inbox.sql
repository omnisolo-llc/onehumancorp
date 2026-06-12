-- +goose Up
-- Migration 118: Update PowerSync Publication with Agent Feed and Omni Inbox

CREATE TABLE IF NOT EXISTS unified_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source_platform TEXT NOT NULL,
    external_id TEXT,
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'unread',
    agent_draft_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agent_action_cards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    unified_message_id TEXT REFERENCES unified_messages(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    proposed_action JSONB,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('unified_messages') IS NOT NULL THEN
        ALTER TABLE unified_messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('agent_action_cards') IS NOT NULL THEN
        ALTER TABLE agent_action_cards ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_agent_action_cards ON agent_action_cards USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_agent_action_cards ON agent_action_cards;
    DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
END
$$;

DROP TABLE IF EXISTS agent_action_cards CASCADE;
DROP TABLE IF EXISTS unified_messages CASCADE;
