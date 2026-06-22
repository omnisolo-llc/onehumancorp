CREATE TABLE IF NOT EXISTS mcp_sync_deltas (
    tenant_id TEXT NOT NULL DEFAULT 'default',
    id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT false,
    PRIMARY KEY (id)
);

ALTER TABLE mcp_sync_deltas ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_mcp_sync_deltas ON mcp_sync_deltas;
CREATE POLICY tenant_isolation_mcp_sync_deltas
ON mcp_sync_deltas
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
