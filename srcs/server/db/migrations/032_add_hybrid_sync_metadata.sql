-- In OHC we use 'autodream_memories' instead of 'rag_memories' according to instructions.
-- So we add these columns to autodream_memories.
ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
