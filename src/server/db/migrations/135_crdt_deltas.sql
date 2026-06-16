-- +goose Up
-- Migration 135: Add crdt_deltas and mcp_sync_deltas

CREATE TABLE IF NOT EXISTS crdt_deltas (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    data TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (tenant_id, id)
);

CREATE TABLE IF NOT EXISTS mcp_sync_deltas (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    updated_at BIGINT NOT NULL,
    PRIMARY KEY (tenant_id, id)
);

ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE mcp_sync_deltas ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_mcp_sync_deltas ON mcp_sync_deltas;
CREATE POLICY tenant_isolation_mcp_sync_deltas ON mcp_sync_deltas USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_mcp_sync_deltas ON mcp_sync_deltas;
DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
DROP TABLE IF EXISTS crdt_deltas CASCADE;
DROP TABLE IF EXISTS mcp_sync_deltas CASCADE;
