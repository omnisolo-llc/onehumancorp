-- +goose Up
-- Migration 130: Add agent_action_drafts table

CREATE TABLE IF NOT EXISTS agent_action_drafts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    message_id TEXT,
    proposed_response TEXT,
    context_used TEXT,
    state TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'approved', 'discarded'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('agent_action_drafts') IS NOT NULL THEN
        ALTER TABLE agent_action_drafts ENABLE ROW LEVEL SECURITY;
        DROP POLICY IF EXISTS tenant_isolation_agent_action_drafts ON agent_action_drafts;
        CREATE POLICY tenant_isolation_agent_action_drafts ON agent_action_drafts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_agent_action_drafts ON agent_action_drafts;
END
$$;

DROP TABLE IF EXISTS agent_action_drafts CASCADE;
