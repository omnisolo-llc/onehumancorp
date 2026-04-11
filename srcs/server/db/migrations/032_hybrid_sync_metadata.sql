-- 032_hybrid_sync_metadata.sql
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMPTZ NULL;

UPDATE autodream_memories SET sync_status = 'synced' WHERE sync_status IS NULL;
