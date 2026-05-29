CREATE TABLE IF NOT EXISTS capital_advances (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    amount FLOAT NOT NULL,
    fee FLOAT NOT NULL,
    repayment_percentage FLOAT NOT NULL,
    status TEXT NOT NULL, -- PENDING, ACTIVE, REPAID
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE capital_advances ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_advances ON capital_advances USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS repayment_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    advance_id TEXT NOT NULL REFERENCES capital_advances(id) ON DELETE CASCADE,
    amount FLOAT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE repayment_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_repayment_events ON repayment_events USING (tenant_id::text = current_setting('app.current_tenant', true));
