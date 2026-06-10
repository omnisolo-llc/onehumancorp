-- +goose Up
-- Migration 115: Add omni_inbox_messages table

CREATE TABLE IF NOT EXISTS omni_inbox_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT NOT NULL,
    original_content TEXT NOT NULL,
    translated_content TEXT NOT NULL,
    source_language TEXT,
    target_language TEXT NOT NULL,
    draft_reply TEXT,
    status TEXT NOT NULL DEFAULT 'unread',
    sender_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('omni_inbox_messages') IS NOT NULL THEN
        ALTER TABLE omni_inbox_messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omni_inbox_messages ON omni_inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_omni_inbox_messages ON omni_inbox_messages;
END
$$;

DROP TABLE IF EXISTS omni_inbox_messages CASCADE;
