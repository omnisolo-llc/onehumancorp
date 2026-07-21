-- +goose Up
-- Migration 157: Universal Agent Feed Core Pipeline

CREATE TABLE IF NOT EXISTS feed_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    source TEXT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agent_action_drafts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    feed_event_id TEXT NOT NULL REFERENCES feed_events(id) ON DELETE CASCADE,
    agent_type TEXT NOT NULL,
    proposed_action JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING', -- PENDING, APPROVED, EDITED, REJECTED
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS action_approvals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_action_draft_id TEXT NOT NULL REFERENCES agent_action_drafts(id) ON DELETE CASCADE,
    decision TEXT NOT NULL, -- APPROVED, EDITED, REJECTED
    edited_action JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('feed_events') IS NOT NULL THEN
        ALTER TABLE feed_events ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_feed_events ON feed_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('agent_action_drafts') IS NOT NULL THEN
        ALTER TABLE agent_action_drafts ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_agent_action_drafts ON agent_action_drafts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('action_approvals') IS NOT NULL THEN
        ALTER TABLE action_approvals ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_action_approvals ON action_approvals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_feed_events ON feed_events;
    DROP POLICY IF EXISTS tenant_isolation_agent_action_drafts ON agent_action_drafts;
    DROP POLICY IF EXISTS tenant_isolation_action_approvals ON action_approvals;
END
$$;

DROP TABLE IF EXISTS action_approvals CASCADE;
DROP TABLE IF EXISTS agent_action_drafts CASCADE;
DROP TABLE IF EXISTS feed_events CASCADE;
