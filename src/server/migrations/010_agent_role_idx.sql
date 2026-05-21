-- Migration: Add agent_role to sub_agent_queue

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='sub_agent_queue' AND column_name='agent_role') THEN
        ALTER TABLE sub_agent_queue ADD COLUMN agent_role VARCHAR;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_role_status ON sub_agent_queue (status, agent_role, scheduled_at);
