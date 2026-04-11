-- 032_rag_sync_metadata.sql
-- Hybrid RAG sync metadata columns for SQLite and Postgres compatibility.

ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
