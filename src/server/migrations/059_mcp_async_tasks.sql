-- +goose Up
CREATE TABLE IF NOT EXISTS mcp_async_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    status TEXT NOT NULL,
    payload TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE mcp_async_tasks ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_mcp_async_tasks ON mcp_async_tasks;
CREATE POLICY tenant_isolation_mcp_async_tasks ON mcp_async_tasks USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_mcp_async_tasks ON mcp_async_tasks;
DROP TABLE IF EXISTS mcp_async_tasks;
