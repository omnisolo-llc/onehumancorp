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
