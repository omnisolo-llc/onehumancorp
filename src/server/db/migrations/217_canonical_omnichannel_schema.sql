-- +goose Up
-- Migration 217: Canonical Omnichannel Schema

CREATE TABLE IF NOT EXISTS omnichannel_inbox (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omnichannel_conversation (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL REFERENCES omnichannel_inbox(id) ON DELETE CASCADE,
    channel TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    priority TEXT NOT NULL DEFAULT 'normal',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('omnichannel_inbox') IS NOT NULL THEN
        ALTER TABLE omnichannel_inbox ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omnichannel_inbox ON omnichannel_inbox USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('omnichannel_conversation') IS NOT NULL THEN
        ALTER TABLE omnichannel_conversation ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_omnichannel_conversation ON omnichannel_conversation USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_omnichannel_inbox ON omnichannel_inbox;
    DROP POLICY IF EXISTS tenant_isolation_omnichannel_conversation ON omnichannel_conversation;
END
$$;

DROP TABLE IF EXISTS omnichannel_conversation CASCADE;
DROP TABLE IF EXISTS omnichannel_inbox CASCADE;
