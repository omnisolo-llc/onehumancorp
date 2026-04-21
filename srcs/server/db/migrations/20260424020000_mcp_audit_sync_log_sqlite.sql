CREATE TABLE IF NOT EXISTS mcp_audit_sync_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tenant_id VARCHAR(255) NOT NULL,
    agent_id VARCHAR(255) NOT NULL,
    action VARCHAR(255) NOT NULL,
    resource VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    metadata TEXT,
    timestamp BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);
