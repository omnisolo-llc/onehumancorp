-- Migration to add sync metadata to swarm_truth_embeddings for Hybrid MCP RAG Protocol
ALTER TABLE swarm_truth_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_truth_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings;
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true));
