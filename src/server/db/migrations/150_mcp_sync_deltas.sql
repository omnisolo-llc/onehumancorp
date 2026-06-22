-- +goose Up

CREATE TABLE IF NOT EXISTS mcp_sync_deltas (
    tenant_id VARCHAR(255) NOT NULL DEFAULT 'default',
    id VARCHAR(255) NOT NULL,
    entity_type VARCHAR(255) NOT NULL,
    entity_id VARCHAR(255) NOT NULL,
    payload JSONB NOT NULL,
    updated_at BIGINT NOT NULL,
    synced_to_cloud BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id)
);

CREATE INDEX IF NOT EXISTS idx_mcp_sync_deltas_updated_at ON mcp_sync_deltas(updated_at);

ALTER TABLE mcp_sync_deltas ENABLE ROW LEVEL SECURITY;
ALTER TABLE mcp_sync_deltas FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_mcp_sync_deltas ON mcp_sync_deltas;
CREATE POLICY tenant_isolation_mcp_sync_deltas
ON mcp_sync_deltas
FOR ALL
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS mcp_sync_deltas;
