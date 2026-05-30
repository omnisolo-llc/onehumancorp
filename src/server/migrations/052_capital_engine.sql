-- Migration 052: Autonomous Working Capital & Micro-Lending Engine

CREATE TABLE IF NOT EXISTS capital_offers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    amount_cents BIGINT NOT NULL,
    fee_cents BIGINT NOT NULL,
    sweep_percentage DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE capital_offers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_offers ON capital_offers USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS capital_advances (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    offer_id TEXT NOT NULL REFERENCES capital_offers(id) ON DELETE CASCADE,
    total_repayment_cents BIGINT NOT NULL,
    repaid_cents BIGINT NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE capital_advances ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_advances ON capital_advances USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS transaction_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    amount_cents BIGINT NOT NULL,
    type TEXT NOT NULL,
    reference_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE transaction_ledger ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_transaction_ledger ON transaction_ledger USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS idx_capital_offers_tenant_id ON capital_offers(tenant_id);
CREATE INDEX IF NOT EXISTS idx_capital_advances_tenant_id ON capital_advances(tenant_id);
CREATE INDEX IF NOT EXISTS idx_transaction_ledger_tenant_id ON transaction_ledger(tenant_id);
