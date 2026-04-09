-- 032_add_hybrid_sync_metadata.sql
-- Add sync metadata to consolidated_memory for Hybrid MCP RAG Protocol

ALTER TABLE consolidated_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE consolidated_memory ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
