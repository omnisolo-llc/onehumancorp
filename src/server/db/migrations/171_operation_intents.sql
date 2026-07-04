CREATE TABLE IF NOT EXISTS operation_intents (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retry_count INT NOT NULL DEFAULT 0
);
ALTER TABLE operation_intents ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_operation_intents ON operation_intents
    USING (tenant_id = current_setting('app.current_tenant', true));
