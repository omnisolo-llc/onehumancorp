-- +goose Up

-- Migration: Unified Work Triage Engine (Postgres)

-- Update unified_messages
ALTER TABLE unified_messages ADD COLUMN IF NOT EXISTS channel TEXT;
ALTER TABLE unified_messages ADD COLUMN IF NOT EXISTS sender_id TEXT;
ALTER TABLE unified_messages ADD COLUMN IF NOT EXISTS raw_payload TEXT;
ALTER TABLE unified_messages ADD COLUMN IF NOT EXISTS normalized_text TEXT;
ALTER TABLE unified_messages ADD COLUMN IF NOT EXISTS status TEXT DEFAULT 'pending';

-- Create action_cards table
CREATE TABLE IF NOT EXISTS action_cards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    message_id TEXT,
    card_type TEXT NOT NULL,
    content_json TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('action_cards') IS NOT NULL THEN
        ALTER TABLE action_cards ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_action_cards ON action_cards;
        CREATE POLICY tenant_isolation_action_cards ON action_cards USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_action_cards ON action_cards;
END
$$;

DROP TABLE IF EXISTS action_cards CASCADE;

ALTER TABLE unified_messages DROP COLUMN IF EXISTS channel;
ALTER TABLE unified_messages DROP COLUMN IF EXISTS sender_id;
ALTER TABLE unified_messages DROP COLUMN IF EXISTS raw_payload;
ALTER TABLE unified_messages DROP COLUMN IF EXISTS normalized_text;
ALTER TABLE unified_messages DROP COLUMN IF EXISTS status;
