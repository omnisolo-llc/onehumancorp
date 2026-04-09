-- Add sync metadata to swarm_memory_embeddings
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMPTZ NULL;

-- If SQLite error handling is an issue we might need some trick, but the instructions
-- say "Ensure the migration uses standard SQL compatible with both PostgreSQL and SQLite. Use `ALTER TABLE ADD COLUMN` appropriately."
