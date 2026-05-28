-- Migration 020: Autonomous Cash Flow & Smart Capital Engine

-- Create cash_flow_predictions table
CREATE TABLE IF NOT EXISTS cash_flow_predictions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    business_id TEXT REFERENCES businesses(id) ON DELETE CASCADE,
    predicted_date DATE NOT NULL,
    predicted_inflow DECIMAL(15, 2) NOT NULL DEFAULT 0,
    predicted_outflow DECIMAL(15, 2) NOT NULL DEFAULT 0,
    predicted_balance DECIMAL(15, 2) NOT NULL DEFAULT 0,
    confidence_score DECIMAL(5, 4) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create capital_offers table
CREATE TABLE IF NOT EXISTS capital_offers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    business_id TEXT REFERENCES businesses(id) ON DELETE CASCADE,
    offer_amount DECIMAL(15, 2) NOT NULL,
    flat_fee_amount DECIMAL(15, 2) NOT NULL,
    flat_fee_percentage DECIMAL(5, 4) NOT NULL,
    repayment_percentage DECIMAL(5, 4) NOT NULL,
    estimated_repayment_days INTEGER NOT NULL,
    total_repayment_amount DECIMAL(15, 2) NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, accepted, active, repaid, declined
    reason TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    accepted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create capital_advances table (accepted offers become advances)
CREATE TABLE IF NOT EXISTS capital_advances (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    business_id TEXT REFERENCES businesses(id) ON DELETE CASCADE,
    offer_id TEXT REFERENCES capital_offers(id) ON DELETE CASCADE,
    principal_amount DECIMAL(15, 2) NOT NULL,
    flat_fee_amount DECIMAL(15, 2) NOT NULL,
    total_owed DECIMAL(15, 2) NOT NULL,
    amount_repaid DECIMAL(15, 2) NOT NULL DEFAULT 0,
    repayment_percentage DECIMAL(5, 4) NOT NULL,
    status TEXT NOT NULL DEFAULT 'active', -- active, repaid, defaulted
    disbursed_at TIMESTAMPTZ NOT NULL,
    repaid_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create capital_repayments table
CREATE TABLE IF NOT EXISTS capital_repayments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    advance_id TEXT REFERENCES capital_advances(id) ON DELETE CASCADE,
    transaction_id TEXT,
    repayment_amount DECIMAL(15, 2) NOT NULL,
    transaction_amount DECIMAL(15, 2) NOT NULL,
    repayment_date TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create sales_transactions table for cash flow tracking
CREATE TABLE IF NOT EXISTS sales_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    business_id TEXT REFERENCES businesses(id) ON DELETE CASCADE,
    transaction_date TIMESTAMPTZ NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    transaction_type TEXT NOT NULL, -- sale, refund, expense
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on all tables
ALTER TABLE cash_flow_predictions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_cash_flow_predictions ON cash_flow_predictions 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE capital_offers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_offers ON capital_offers 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE capital_advances ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_advances ON capital_advances 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE capital_repayments ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capital_repayments ON capital_repayments 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE sales_transactions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_sales_transactions ON sales_transactions 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_cash_flow_predictions_tenant ON cash_flow_predictions(tenant_id, predicted_date);
CREATE INDEX IF NOT EXISTS idx_capital_offers_tenant_status ON capital_offers(tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_capital_advances_tenant_status ON capital_advances(tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_sales_transactions_tenant_date ON sales_transactions(tenant_id, transaction_date);
