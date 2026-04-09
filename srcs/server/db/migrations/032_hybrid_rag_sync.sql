-- +goose Up
-- Add sync metadata to consolidated_memory for Hybrid RAG
ALTER TABLE consolidated_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE consolidated_memory ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- Remove sync metadata
ALTER TABLE consolidated_memory DROP COLUMN sync_status;
ALTER TABLE consolidated_memory DROP COLUMN last_sync_at;
