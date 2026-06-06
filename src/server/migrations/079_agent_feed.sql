-- +goose Up
-- Migration 079: Add status column to agent_actions table

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'agent_actions'
        AND column_name = 'status'
    ) THEN
        ALTER TABLE agent_actions ADD COLUMN status TEXT NOT NULL DEFAULT 'PENDING';
    END IF;
END
$$;

CREATE INDEX IF NOT EXISTS idx_agent_actions_tenant_status ON agent_actions(tenant_id, status);

-- +goose Down
DROP INDEX IF EXISTS idx_agent_actions_tenant_status;
ALTER TABLE agent_actions DROP COLUMN IF EXISTS status;
