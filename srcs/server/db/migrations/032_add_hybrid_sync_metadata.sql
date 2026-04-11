-- +goose Up
-- Add sync metadata to swarm_memory_embeddings for Hybrid MCP RAG Protocol
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP;

-- +goose Down
-- Remove sync metadata from swarm_memory_embeddings
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN sync_status;
-- ALTER TABLE swarm_memory_embeddings DROP COLUMN last_sync_at;
