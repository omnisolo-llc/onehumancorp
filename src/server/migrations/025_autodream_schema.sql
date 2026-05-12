-- 025_autodream_schema.sql

CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_mission_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
