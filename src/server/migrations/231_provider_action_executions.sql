-- Migration 231: Provider action executions

CREATE TABLE IF NOT EXISTS provider_action_executions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
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
