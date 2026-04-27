CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS consolidated_memory (
    id SERIAL PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    content TEXT,
    embedding VECTOR(1536)
);
ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
