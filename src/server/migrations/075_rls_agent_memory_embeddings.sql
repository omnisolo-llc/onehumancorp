ALTER TABLE agent_memory_embeddings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_memory_embeddings_strict ON agent_memory_embeddings;
CREATE POLICY tenant_isolation_agent_memory_embeddings_strict ON agent_memory_embeddings
    USING (organization_id::text = current_setting('app.current_tenant', true));
