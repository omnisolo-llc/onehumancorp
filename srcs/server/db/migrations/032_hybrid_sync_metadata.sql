-- Migration to add hybrid sync metadata for MCP RAG Protocol.
-- Since SQLite does not support multiple ADD COLUMN statements or IF NOT EXISTS,
-- we use standard individual ALTER TABLE statements.

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
