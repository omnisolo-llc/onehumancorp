-- The agent feed repository unions this table with the other approval sources.
-- Keep this migration forward-only: this directory is executed by SQLx.

CREATE TABLE IF NOT EXISTS agent_action_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT,
    action_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    confidence_score DECIMAL DEFAULT 0,
    product_id TEXT,
    payload JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    source TEXT,
    agent_type TEXT
);

ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS action_type TEXT;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS status TEXT DEFAULT 'Pending';
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS confidence_score DECIMAL DEFAULT 0;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS product_id TEXT;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS payload JSONB DEFAULT '{}'::jsonb;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS source TEXT;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS agent_type TEXT;

CREATE INDEX IF NOT EXISTS idx_agent_action_requests_tenant_status
    ON agent_action_requests (tenant_id, status, created_at DESC);

ALTER TABLE agent_action_requests ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'agent_action_requests'
          AND policyname = 'tenant_isolation_agent_action_requests'
    ) THEN
        CREATE POLICY tenant_isolation_agent_action_requests
            ON agent_action_requests
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
