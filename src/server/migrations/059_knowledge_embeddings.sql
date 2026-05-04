-- 059_knowledge_embeddings.sql

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS knowledge_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    agent_id VARCHAR,
    task_id VARCHAR,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_type VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_knowledge_embeddings_ivfflat ON knowledge_embeddings USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);

ALTER TABLE knowledge_embeddings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_knowledge_embeddings ON knowledge_embeddings USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
