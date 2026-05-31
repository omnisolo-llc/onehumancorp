-- Migration 054: Autonomous Embedded Capital Engine

CREATE TABLE IF NOT EXISTS capital_offers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    merchant_id TEXT NOT NULL,
    amount DECIMAL NOT NULL,
    flat_fee DECIMAL NOT NULL,
    repayment_percentage DECIMAL NOT NULL,
    status TEXT NOT NULL,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE capital_offers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_offers ON capital_offers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS capital_advances (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    offer_id TEXT NOT NULL REFERENCES capital_offers(id) ON DELETE CASCADE,
    total_owed DECIMAL NOT NULL,
    total_repaid DECIMAL NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE capital_advances ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_advances ON capital_advances USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS repayment_splits (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    advance_id TEXT NOT NULL REFERENCES capital_advances(id) ON DELETE CASCADE,
    transaction_id TEXT NOT NULL,
    amount DECIMAL NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE repayment_splits ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_repayment_splits ON repayment_splits USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
