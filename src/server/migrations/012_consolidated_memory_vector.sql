CREATE EXTENSION IF NOT EXISTS vector;
CREATE INDEX IF NOT EXISTS consolidated_memory_embedding_idx ON consolidated_memory USING hnsw (embedding vector_cosine_ops);
