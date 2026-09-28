-- +goose Up
CREATE TABLE IF NOT EXISTS provider_action_executions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    action_request_id TEXT NOT NULL REFERENCES agent_action_requests(id) ON DELETE CASCADE,
    idempotency_key TEXT NOT NULL,
    provider_name TEXT NOT NULL,
    execution_status TEXT NOT NULL DEFAULT 'pending_execution',
    provider_receipt_id TEXT,
    failure_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, idempotency_key)
);

CREATE INDEX IF NOT EXISTS idx_provider_action_executions_tenant ON provider_action_executions(tenant_id);

DO $$
BEGIN
    IF to_regclass('provider_action_executions') IS NOT NULL THEN
        ALTER TABLE provider_action_executions ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'provider_action_executions'
                AND policyname = 'tenant_isolation_provider_action_executions'
        ) THEN
            CREATE POLICY tenant_isolation_provider_action_executions ON provider_action_executions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('provider_action_executions') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_provider_action_executions ON provider_action_executions;
        ALTER TABLE provider_action_executions DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS provider_action_executions CASCADE;
