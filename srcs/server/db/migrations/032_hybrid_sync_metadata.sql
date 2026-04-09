-- 032_hybrid_sync_metadata.sql

ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMPTZ NULL;

ALTER TABLE consolidated_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE consolidated_memory ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
