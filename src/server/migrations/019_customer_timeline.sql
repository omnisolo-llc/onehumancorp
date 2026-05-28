CREATE TABLE IF NOT EXISTS customer_timeline (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
    agent_id TEXT,
    action_type TEXT NOT NULL,
    description TEXT NOT NULL,
    payload JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_customer_timeline_tenant_customer ON customer_timeline(tenant_id, customer_id);
