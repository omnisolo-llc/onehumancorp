-- +goose Up
-- Migration 146: Create offline_mutation_log table for idempotency

CREATE TABLE IF NOT EXISTS offline_mutation_log (
    client_mutation_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    mutation_type TEXT,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_offline_mutation_log_tenant
ON offline_mutation_log(tenant_id);

ALTER TABLE offline_mutation_log ENABLE ROW LEVEL SECURITY;
ALTER TABLE offline_mutation_log FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_offline_mutation_log ON offline_mutation_log;
CREATE POLICY tenant_isolation_offline_mutation_log
ON offline_mutation_log
FOR ALL
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS offline_mutation_log;
