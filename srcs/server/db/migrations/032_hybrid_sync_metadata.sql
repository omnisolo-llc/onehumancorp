-- Add hybrid sync metadata to swarm_memory_embeddings for Hybrid MCP RAG Protocol
-- Compatible with both PostgreSQL and SQLite (no IF NOT EXISTS on SQLite)
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
