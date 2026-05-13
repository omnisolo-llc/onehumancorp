-- 045_crdt_deltas.sql
CREATE TABLE IF NOT EXISTS crdt_deltas (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    data TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (tenant_id, id)
);
ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS local_mcp_rag_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    escalation_status TEXT NOT NULL
);
ALTER TABLE local_mcp_rag_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_local_mcp_rag_tasks ON local_mcp_rag_tasks USING (tenant_id = current_setting('app.current_tenant', true));
