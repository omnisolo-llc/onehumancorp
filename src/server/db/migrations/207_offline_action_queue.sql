-- Migration 207: Offline Action Queue

CREATE TABLE IF NOT EXISTS offline_action_queue (
    id VARCHAR(255) PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL REFERENCES tenants(id),
    idempotency_key VARCHAR(255) NOT NULL,
    action_type VARCHAR(255) NOT NULL,
    payload JSONB NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

ALTER TABLE offline_action_queue ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_offline_action_queue ON offline_action_queue
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

CREATE UNIQUE INDEX idx_offline_action_queue_idempotency ON offline_action_queue(tenant_id, idempotency_key);
CREATE INDEX idx_offline_action_queue_tenant_status ON offline_action_queue(tenant_id, status);
