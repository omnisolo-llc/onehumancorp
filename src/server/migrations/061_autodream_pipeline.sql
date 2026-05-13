-- 061_autodream_pipeline.sql

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_type TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1,
    sync_status VARCHAR(50) DEFAULT 'pending',
    last_sync_at TIMESTAMP WITH TIME ZONE NULL,
    topic TEXT NOT NULL DEFAULT ''
);
ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (organization_id = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS idx_autodream_org ON autodream_memories(organization_id);
CREATE INDEX IF NOT EXISTS idx_autodream_memories_embedding_cosine ON autodream_memories USING ivfflat (embedding vector_cosine_ops);
