CREATE TABLE IF NOT EXISTS local_mcp_rag_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    payload TEXT,
    escalation_status TEXT
);
ALTER TABLE local_mcp_rag_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY local_mcp_rag_tasks_isolation_policy ON local_mcp_rag_tasks
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
