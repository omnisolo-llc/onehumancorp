-- 025_autodream_schema.sql

CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_mission_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;

ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
