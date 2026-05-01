-- 051_autodream_memories.sql

CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    task_id UUID REFERENCES shared_tasks_decomposition(id),
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    agent_id TEXT,
    source_type TEXT,
    topic TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_autodream_memories_embedding ON autodream_memories USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
