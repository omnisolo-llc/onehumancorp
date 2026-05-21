CREATE TABLE IF NOT EXISTS mcp_audit_sync_log (
    id SERIAL PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    agent_id VARCHAR(255) NOT NULL,
    action VARCHAR(255) NOT NULL,
    resource VARCHAR(255),
    status VARCHAR(50),
    metadata TEXT,
    timestamp BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_mcp_audit_sync_log_tenant_id ON mcp_audit_sync_log(tenant_id);
CREATE INDEX IF NOT EXISTS idx_mcp_audit_sync_log_agent_id ON mcp_audit_sync_log(agent_id);
