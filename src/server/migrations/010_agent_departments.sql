CREATE TABLE IF NOT EXISTS agent_departments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    department_type TEXT NOT NULL,
    config JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, department_type)
);
ALTER TABLE agent_departments ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_departments ON agent_departments USING (tenant_id::text = current_setting('app.current_tenant', true));
