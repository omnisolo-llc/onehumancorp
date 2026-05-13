CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    embedding TEXT,
    source_mission_id TEXT
);
