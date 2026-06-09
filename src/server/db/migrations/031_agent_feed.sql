-- +goose Up
-- Migration 031: Add Agent Feed tables

CREATE TABLE IF NOT EXISTS agent_feed_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    event_source TEXT NOT NULL,
    context_payload JSONB,
    proposed_action JSONB,
    lifecycle_state TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('agent_feed_items') IS NOT NULL THEN
        ALTER TABLE agent_feed_items ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_agent_feed_items ON agent_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_agent_feed_items ON agent_feed_items;
END
$$;

DROP TABLE IF EXISTS agent_feed_items CASCADE;
