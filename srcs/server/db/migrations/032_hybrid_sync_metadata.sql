-- +goose Up
-- Add sync_status and last_sync_at to swarm_memory_embeddings for Hybrid MCP RAG Protocol
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- Downgrade is ignored for SQLite compatibility with ALTER TABLE ADD COLUMN.
-- In pure Postgres, it would be:
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN sync_status;
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN last_sync_at;
