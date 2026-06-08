-- +goose Up
-- Migration 108: Autonomous Agent Inventory CRM Pricing Actions

CREATE TABLE IF NOT EXISTS agent_action_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    confidence_score DOUBLE PRECISION NOT NULL,
    department TEXT,
    description TEXT,
    product_id TEXT,
    suggested_quantity INT,
    suggested_price_cents BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_action_requests_tenant ON agent_action_requests(tenant_id);

ALTER TABLE agent_action_requests ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_agent_action_requests ON agent_action_requests;
CREATE POLICY tenant_isolation_agent_action_requests ON agent_action_requests
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS agent_action_requests CASCADE;
