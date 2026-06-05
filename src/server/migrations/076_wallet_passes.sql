CREATE TABLE IF NOT EXISTS wallet_passes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    token TEXT UNIQUE NOT NULL,
    pass_type TEXT NOT NULL,
    payload JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_wallet_passes_tenant_customer ON wallet_passes(tenant_id, customer_id);

ALTER TABLE wallet_passes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_wallet_passes ON wallet_passes;
CREATE POLICY tenant_isolation_wallet_passes
ON wallet_passes
FOR ALL
USING (tenant_id = current_setting('app.current_tenant')::text);
