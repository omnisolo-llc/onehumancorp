ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS agent_role VARCHAR;
CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_polling ON sub_agent_queue(status, scheduled_at, agent_role) WHERE status = 'PENDING';

UPDATE sub_agent_queue SET agent_role = payload::json->>'agent_role' WHERE status = 'PENDING';
