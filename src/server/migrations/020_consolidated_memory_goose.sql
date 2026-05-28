-- +goose Up
-- Migration 020: Goose migrations for consolidated_memory

CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    reference_count INTEGER DEFAULT 0,
    reliability_score INTEGER DEFAULT 50,
    owner_override BOOLEAN DEFAULT FALSE,
    metadata JSONB
);

ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE INDEX IF NOT EXISTS consolidated_memory_embedding_hnsw_idx ON consolidated_memory USING hnsw (embedding vector_cosine_ops);

-- +goose Down
-- Reverse Migration 020

DROP INDEX IF EXISTS consolidated_memory_embedding_hnsw_idx;
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
-- DROP TABLE IF EXISTS consolidated_memory CASCADE;
