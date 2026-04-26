CREATE TABLE IF NOT EXISTS mcp_config_sync_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    metadata TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_mcp_config_sync_log_tenant_key ON mcp_config_sync_log(tenant_id, key);
