-- 032_add_hybrid_sync_metadata.sql
-- Add hybrid sync metadata to consolidated_memory for RAG Sync Protocol

ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMP NULL;
