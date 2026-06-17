ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE swarm_truth_embeddings SET tenant_id = 'default' WHERE tenant_id IS NULL;
ALTER TABLE swarm_truth_embeddings ALTER COLUMN tenant_id SET NOT NULL;
CREATE INDEX IF NOT EXISTS idx_swarm_truth_embeddings_tenant_id ON swarm_truth_embeddings(tenant_id);

DROP POLICY IF EXISTS tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings;
CREATE POLICY tenant_isolation_swarm_truth_embeddings
    ON swarm_truth_embeddings
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
