-- +goose Up
-- Migration 217: Add Native Omnichannel Chat Tables

CREATE TABLE IF NOT EXISTS inbox (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    config JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

DO $$
BEGIN
    IF to_regclass('inbox') IS NOT NULL THEN
        ALTER TABLE inbox ENABLE ROW LEVEL SECURITY;
        CREATE POLICY inbox_tenant_isolation_policy ON inbox FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS conversation (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES inbox(id),
    contact_id UUID NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

DO $$
BEGIN
    IF to_regclass('conversation') IS NOT NULL THEN
        ALTER TABLE conversation ENABLE ROW LEVEL SECURITY;
        CREATE POLICY conversation_tenant_isolation_policy ON conversation FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
    END IF;
END
$$;


CREATE TABLE IF NOT EXISTS message (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES conversation(id),
    content TEXT NOT NULL,
    content_type TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

DO $$
BEGIN
    IF to_regclass('message') IS NOT NULL THEN
        ALTER TABLE message ENABLE ROW LEVEL SECURITY;
        CREATE POLICY message_tenant_isolation_policy ON message FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS message_tenant_isolation_policy ON message;
    DROP POLICY IF EXISTS conversation_tenant_isolation_policy ON conversation;
    DROP POLICY IF EXISTS inbox_tenant_isolation_policy ON inbox;
END
$$;

DROP TABLE IF EXISTS message CASCADE;
DROP TABLE IF EXISTS conversation CASCADE;
DROP TABLE IF EXISTS inbox CASCADE;
