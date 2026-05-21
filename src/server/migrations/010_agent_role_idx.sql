-- Migration: Add agent_role to sub_agent_queue
ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS agent_role VARCHAR;
CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_role_status ON sub_agent_queue (status, agent_role, scheduled_at);
