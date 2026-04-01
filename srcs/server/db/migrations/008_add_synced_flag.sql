-- 008_add_synced_flag.sql
-- Add synced flag to swarm_memory_embeddings for Hybrid RAG state sync

ALTER TABLE swarm_memory_embeddings ADD COLUMN synced BOOLEAN DEFAULT FALSE;
