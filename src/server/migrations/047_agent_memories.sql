-- 047_agent_memories.sql

CREATE EXTENSION IF NOT EXISTS vector;

ALTER TABLE tasks ADD COLUMN IF NOT EXISTS auto_dreamed BOOLEAN DEFAULT FALSE;

CREATE TABLE IF NOT EXISTS agent_memories (
    id UUID PRIMARY KEY,
    organization_id UUID,
    task_id UUID REFERENCES tasks(id) ON DELETE CASCADE,
    raw_content TEXT,
    summary_embedding VECTOR(1536),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
