CREATE TABLE IF NOT EXISTS order_fulfillment_state (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    status TEXT NOT NULL,
    crdt_clock BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE order_fulfillment_state ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_order_fulfillment_state ON order_fulfillment_state
    USING (tenant_id = current_setting('app.current_tenant', TRUE));
