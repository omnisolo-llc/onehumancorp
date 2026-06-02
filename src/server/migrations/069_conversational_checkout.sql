CREATE TABLE IF NOT EXISTS conversational_checkout_session (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    deposit_amount FLOAT,
    inventory_lock_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_conversational_checkout_session_tenant_id ON conversational_checkout_session(tenant_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_conversational_checkout_session_customer_id ON conversational_checkout_session(customer_id);

ALTER TABLE conversational_checkout_session ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conversational_checkout_session ON conversational_checkout_session;
CREATE POLICY tenant_isolation_conversational_checkout_session ON conversational_checkout_session USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
