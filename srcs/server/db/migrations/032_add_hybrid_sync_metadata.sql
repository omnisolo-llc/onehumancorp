-- Add hybrid sync metadata columns to rag_memories table
-- Using standard SQL compatible with PostgreSQL and SQLite

ALTER TABLE rag_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE rag_memories ADD COLUMN last_sync_at TIMESTAMP NULL;
