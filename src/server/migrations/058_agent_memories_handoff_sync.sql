-- 058_agent_memories_handoff_sync.sql

ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS raw_content BYTEA;
ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP;
