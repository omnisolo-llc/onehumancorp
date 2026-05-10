-- +goose Up
-- Add sync metadata columns to memory tables for Hybrid MCP RAG Protocol
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
ALTER TABLE swarm_memory_embeddings DROP COLUMN IF EXISTS last_sync_at;
ALTER TABLE swarm_memory_embeddings DROP COLUMN IF EXISTS sync_status;
