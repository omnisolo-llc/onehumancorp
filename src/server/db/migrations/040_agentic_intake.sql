CREATE TABLE IF NOT EXISTS agentic_intake_flows (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    required_fields JSONB NOT NULL DEFAULT '[]'::jsonb,
    initial_prompt TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS intake_sessions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    flow_id TEXT NOT NULL REFERENCES agentic_intake_flows(id) ON DELETE CASCADE,
    customer_info JSONB DEFAULT '{}'::jsonb,
    collected_data JSONB DEFAULT '{}'::jsonb,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS intake_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    session_id TEXT NOT NULL REFERENCES intake_sessions(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agentic_intake_flows_tenant ON agentic_intake_flows(tenant_id);
CREATE INDEX IF NOT EXISTS idx_intake_sessions_tenant ON intake_sessions(tenant_id);
CREATE INDEX IF NOT EXISTS idx_intake_messages_tenant ON intake_messages(tenant_id);
CREATE INDEX IF NOT EXISTS idx_intake_messages_session ON intake_messages(session_id);

ALTER TABLE agentic_intake_flows ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agentic_intake_flows ON agentic_intake_flows;
CREATE POLICY tenant_isolation_agentic_intake_flows ON agentic_intake_flows USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE intake_sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_intake_sessions ON intake_sessions;
CREATE POLICY tenant_isolation_intake_sessions ON intake_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE intake_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_intake_messages ON intake_messages;
CREATE POLICY tenant_isolation_intake_messages ON intake_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
