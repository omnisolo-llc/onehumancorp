-- Migration to add sync metadata to swarm_truth_embeddings for Hybrid MCP RAG Protocol
ALTER TABLE swarm_truth_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_truth_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
