CREATE TABLE IF NOT EXISTS agent_department_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    department VARCHAR(255) NOT NULL,
    mode VARCHAR(50) NOT NULL DEFAULT 'auto',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, department)
);

ALTER TABLE agent_department_config ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy ON agent_department_config
    USING (tenant_id::text = current_setting('app.current_tenant', true));
