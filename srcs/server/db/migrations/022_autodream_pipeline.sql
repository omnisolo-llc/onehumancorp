-- 022_autodream_pipeline.sql

CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_mission_id TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
