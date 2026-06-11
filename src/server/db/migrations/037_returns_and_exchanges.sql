-- Migration: Omnichannel Returns & Exchange Orchestrator

CREATE TABLE IF NOT EXISTS return_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    reason TEXT NOT NULL,
    action_type TEXT NOT NULL, -- 'refund' or 'exchange'
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'approved', 'rejected', 'processed'
    refund_amount_cents BIGINT NOT NULL DEFAULT 0,
    payment_intent_id TEXT,
    stripe_refund_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_return_requests_tenant ON return_requests(tenant_id);

ALTER TABLE return_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_return_requests ON return_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
