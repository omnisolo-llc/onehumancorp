-- 032_add_hybrid_sync_metadata.sql
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_timestamp TIMESTAMP NULL;
