CREATE TABLE IF NOT EXISTS saga_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    saga_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'running',
    context JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT valid_status CHECK (status IN ('running', 'completed', 'compensating', 'failed'))
);

ALTER TABLE saga_executions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_saga_executions ON saga_executions
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS saga_steps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    saga_id UUID NOT NULL REFERENCES saga_executions(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    step_name TEXT NOT NULL,
    agent_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    retry_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT valid_step_status CHECK (status IN ('pending', 'running', 'completed', 'failed', 'compensated'))
);

ALTER TABLE saga_steps ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_saga_steps ON saga_steps
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
