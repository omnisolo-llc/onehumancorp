-- +goose Up
DROP TABLE IF EXISTS mcp_audit_sync_log CASCADE;

-- +goose Down
CREATE TABLE IF NOT EXISTS mcp_audit_sync_log (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    action TEXT NOT NULL,
    resource TEXT NOT NULL,
    status TEXT NOT NULL,
    metadata TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL
);

ALTER TABLE mcp_audit_sync_log ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_mcp_audit_sync_log ON mcp_audit_sync_log;
