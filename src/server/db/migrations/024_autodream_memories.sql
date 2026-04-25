CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id TEXT,
    task_id TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_mission_id TEXT,
    source_type TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
