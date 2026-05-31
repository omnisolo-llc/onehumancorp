CREATE TABLE IF NOT EXISTS cashflow_forecasts (
    forecast_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    target_date DATE NOT NULL,
    expected_inflow DOUBLE PRECISION NOT NULL,
    expected_outflow DOUBLE PRECISION NOT NULL,
    net_position DOUBLE PRECISION NOT NULL,
    risk_level TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_cashflow_forecasts_tenant_id ON cashflow_forecasts(tenant_id);

CREATE TABLE IF NOT EXISTS capital_offers (
    offer_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    forecast_id TEXT REFERENCES cashflow_forecasts(forecast_id),
    amount DOUBLE PRECISION NOT NULL,
    fee_percentage DOUBLE PRECISION NOT NULL,
    repayment_rate DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_capital_offers_tenant_id ON capital_offers(tenant_id);

ALTER TABLE cashflow_forecasts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_cashflow_forecasts ON cashflow_forecasts USING (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE capital_offers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_offers ON capital_offers USING (tenant_id = current_setting('app.current_tenant', true));
