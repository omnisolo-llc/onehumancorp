-- +goose Up
-- Add sync metadata columns required for Hybrid MCP RAG Protocol.
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;

-- +goose Down
-- In SQLite drop column is supported in modern versions, but standard is to ignore or recreate.
-- We use standard ALTER TABLE DROP COLUMN if supported by postgres.
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS sync_status;
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS last_sync_at;
