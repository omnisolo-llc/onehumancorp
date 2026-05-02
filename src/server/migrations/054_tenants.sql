-- 054_tenants.sql

CREATE TABLE IF NOT EXISTS tenants (
    id              TEXT PRIMARY KEY,
    business_name   TEXT NOT NULL,
    business_type   TEXT NOT NULL,
    flags           JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tenant_agents (
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    agent_id  TEXT NOT NULL,
    role      TEXT NOT NULL,
    PRIMARY KEY (tenant_id, agent_id)
);

ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenant_agents ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_tenants ON tenants USING (id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
CREATE POLICY tenant_isolation_tenant_agents ON tenant_agents USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
