-- +goose Up
-- Migration 142: Unified Inbox Schema

CREATE TABLE IF NOT EXISTS unified_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    sender_id TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'unread',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS pending_work_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    message_id TEXT NOT NULL REFERENCES unified_messages(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
DO $$
BEGIN
    IF to_regclass('unified_messages') IS NOT NULL THEN
        ALTER TABLE unified_messages ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
        CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('pending_work_actions') IS NOT NULL THEN
        ALTER TABLE pending_work_actions ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_pending_work_actions ON pending_work_actions;
        CREATE POLICY tenant_isolation_pending_work_actions ON pending_work_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_pending_work_actions ON pending_work_actions;
    DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
END
$$;

DROP TABLE IF EXISTS pending_work_actions CASCADE;
DROP TABLE IF EXISTS unified_messages CASCADE;
