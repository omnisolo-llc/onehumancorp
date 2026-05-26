ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS sync_error TEXT;
ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMP;
ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS escalation_required INTEGER DEFAULT 0;
ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS sync_status TEXT DEFAULT 'PENDING';
