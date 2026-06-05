-- +goose Up
CREATE TABLE IF NOT EXISTS wallet_passes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customer360(id) ON DELETE CASCADE,
    pass_type TEXT NOT NULL,
    status TEXT NOT NULL,
    pass_data JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_wallet_passes_tenant_customer ON wallet_passes(tenant_id, customer_id);
CREATE INDEX IF NOT EXISTS idx_wallet_passes_status ON wallet_passes(tenant_id, status);

ALTER TABLE wallet_passes ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_wallet_passes ON wallet_passes
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_wallet_passes ON wallet_passes;
DROP INDEX IF EXISTS idx_wallet_passes_tenant_customer;
DROP INDEX IF EXISTS idx_wallet_passes_status;
DROP TABLE IF EXISTS wallet_passes;
