-- 032_hybrid_rag_sync_metadata.sql
-- Add hybrid sync metadata columns to autodream_memories

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50);
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP;

UPDATE autodream_memories SET sync_status = 'pending' WHERE sync_status IS NULL;

-- Cannot easily add DEFAULT 'pending' retrospectively with ALTER TABLE in SQLite without recreating the table,
-- but the prompt requests using ALTER TABLE ADD COLUMN.
