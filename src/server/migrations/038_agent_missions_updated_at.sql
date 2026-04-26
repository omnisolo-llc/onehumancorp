-- 038_agent_missions_updated_at.sql
-- Add updated_at column to agent_missions from Go migration 031

ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ;
UPDATE agent_missions SET updated_at = created_at WHERE updated_at IS NULL;
