-- Migration 039: consolidated_memory and pgvector

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    reference_count INTEGER DEFAULT 0,
    reliability_score INTEGER DEFAULT 50,
    owner_override BOOLEAN DEFAULT FALSE,
    metadata TEXT
);

CREATE INDEX IF NOT EXISTS consolidated_memory_tenant_id_idx ON consolidated_memory(tenant_id);
CREATE INDEX IF NOT EXISTS consolidated_memory_embedding_idx ON consolidated_memory USING hnsw (embedding vector_cosine_ops);
