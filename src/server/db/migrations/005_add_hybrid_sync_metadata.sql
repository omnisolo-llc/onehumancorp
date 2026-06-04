-- Migration to add sync metadata to swarm_truth_embeddings for Hybrid MCP RAG Protocol
ALTER TABLE swarm_truth_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_truth_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;

ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'swarm_truth_embeddings'
          AND policyname = 'tenant_isolation_swarm_truth_embeddings'
    ) THEN
        CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
