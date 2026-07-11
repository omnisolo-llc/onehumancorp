-- Add agent_role to sub_agent_queue
ALTER TABLE sub_agent_queue ADD COLUMN agent_role VARCHAR(255);

-- Backfill agent_role
UPDATE sub_agent_queue
SET agent_role = payload::json->>'agent_role'
WHERE payload::json->>'agent_role' IS NOT NULL;

-- Create index to avoid full table scans on dequeue
CREATE INDEX idx_sub_agent_queue_role_schedule ON sub_agent_queue(status, agent_role, scheduled_at);
