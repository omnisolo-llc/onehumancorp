-- Migration: Add agent_role to sub_agent_queue

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name='sub_agent_queue') THEN
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='sub_agent_queue' AND column_name='agent_role') THEN
            ALTER TABLE sub_agent_queue ADD COLUMN agent_role VARCHAR;
        END IF;
    END IF;
END $$;

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name='sub_agent_queue') THEN
        IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE tablename = 'sub_agent_queue' AND indexname = 'idx_sub_agent_queue_role_status') THEN
            CREATE INDEX idx_sub_agent_queue_role_status ON sub_agent_queue (status, agent_role, scheduled_at);
        END IF;
    END IF;
END $$;
