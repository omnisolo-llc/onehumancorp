-- +goose Up
CREATE TABLE IF NOT EXISTS mcp_servers (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    command TEXT NOT NULL,
    args JSONB NOT NULL DEFAULT '[]',
    env JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_mcp_servers_tenant ON mcp_servers(tenant_id);
CREATE INDEX IF NOT EXISTS idx_mcp_servers_user ON mcp_servers(user_id);

-- +goose Down
DROP TABLE IF EXISTS mcp_servers;
