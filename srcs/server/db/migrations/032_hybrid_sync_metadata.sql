-- +goose Up
-- Add hybrid sync metadata to autodream_memories.
-- We use autodream_memories as the primary context/RAG table since there is no rag_memories.

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP;

-- Populate existing rows to be synced if needed
UPDATE autodream_memories SET sync_status = 'synced' WHERE sync_status IS NULL;

-- +goose Down
-- Since SQLite doesn't natively support dropping columns easily, we omit the downward column drop for SQLite cross-compatibility as per constraints.
