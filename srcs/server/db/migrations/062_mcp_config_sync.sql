CREATE TABLE IF NOT EXISTS mcp_config_sync_log (
    id SERIAL PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    agent_id VARCHAR(255),
    key VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    metadata TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_mcp_config_sync_log_tenant_key ON mcp_config_sync_log (tenant_id, key);
