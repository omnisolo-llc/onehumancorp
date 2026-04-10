-- 032_hybrid_sync_metadata.sql
-- Add hybrid sync metadata to consolidated_memory

ALTER TABLE consolidated_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE consolidated_memory ADD COLUMN last_sync_at TIMESTAMP;
