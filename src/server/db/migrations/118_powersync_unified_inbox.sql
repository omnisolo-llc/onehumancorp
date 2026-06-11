-- +goose Up
-- Migration 118: Update PowerSync Publication with Agent Feed and Omni Inbox

DO $$
BEGIN
    -- We assume the publication 'powersync' already exists from init-multiple-databases.sh
    -- We add agent_feed_items and omni_inbox_messages to the replication.

    -- NOTE: In init-multiple-databases.sh, it creates publication 'powersync' FOR ALL TABLES.
    -- We'll just make sure these tables have REPLICA IDENTITY FULL for PowerSync if required.
    -- PowerSync requires either primary key or replica identity full.
    -- Both tables have a primary key 'id', so we might not need to do anything specifically for publication
    -- other than ensuring they are created.
    -- We will create unified_messages and agent_action_cards if they are explicitly requested instead of omni_inbox_messages and agent_feed_items
END
$$;

-- Actually, the prompt says: "Define the SQL schema and replication rules (Sync Rules) required to synchronize the `unified_messages` and `agent_action_cards` tables to the mobile client securely, ensuring strict `tenant_id` isolation."
-- So I should CREATE `unified_messages` and `agent_action_cards`!

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
