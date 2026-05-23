CREATE TABLE IF NOT EXISTS mcp_audit_sync_log (
    id SERIAL PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    agent_id VARCHAR NOT NULL,
    action VARCHAR NOT NULL,
    resource VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    metadata TEXT NOT NULL,
    timestamp BIGINT NOT NULL
);
