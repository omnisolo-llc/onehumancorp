CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS agent_memory_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    agent_id VARCHAR NOT NULL,
    memory_type VARCHAR NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE agent_memory_embeddings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_memory_embeddings ON agent_memory_embeddings USING (organization_id = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS idx_agent_memory_embeddings ON agent_memory_embeddings USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
