-- +goose Up
CREATE TABLE IF NOT EXISTS agent_action_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    status TEXT NOT NULL,
    confidence_score DOUBLE PRECISION,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_action_requests_tenant ON agent_action_requests(tenant_id);

DO $$
BEGIN
    IF to_regclass('agent_action_requests') IS NOT NULL THEN
        ALTER TABLE agent_action_requests ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_agent_action_requests ON agent_action_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_agent_action_requests ON agent_action_requests;
END
$$;

DROP TABLE IF EXISTS agent_action_requests CASCADE;
