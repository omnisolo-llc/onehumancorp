-- Migration 020: Autonomous Invisible Bookkeeping & Tax Engine
-- This migration creates the foundation for automatic transaction capture,
-- ledger management, and tax calculation for small business owners.

-- Transactions table: captures every financial event
CREATE TABLE IF NOT EXISTS financial_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    transaction_type TEXT NOT NULL, -- 'sale', 'expense', 'refund', 'adjustment'
    amount_cents BIGINT NOT NULL,
    currency TEXT DEFAULT 'USD',
    description TEXT,
    category TEXT, -- auto-categorized: 'inventory', 'supplies', 'marketing', 'labor', etc.
    tax_category TEXT, -- 'taxable_income', 'deductible_expense', 'exempt', etc.
    customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL,
    order_id TEXT REFERENCES orders(id) ON DELETE SET NULL,
    receipt_image_url TEXT, -- for OCR-captured receipts
    receipt_metadata JSONB DEFAULT '{}', -- extracted data from receipt
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Ledger entries: double-entry bookkeeping
CREATE TABLE IF NOT EXISTS ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    transaction_id TEXT REFERENCES financial_transactions(id) ON DELETE CASCADE,
    account_type TEXT NOT NULL, -- 'asset', 'liability', 'equity', 'revenue', 'expense'
    account_name TEXT NOT NULL, -- 'cash', 'accounts_receivable', 'sales_revenue', 'cogs', etc.
    debit_cents BIGINT DEFAULT 0,
    credit_cents BIGINT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Tax obligations: automatically calculated and reserved
CREATE TABLE IF NOT EXISTS tax_obligations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    tax_type TEXT NOT NULL, -- 'sales_tax', 'income_tax', 'self_employment_tax'
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    taxable_amount_cents BIGINT NOT NULL,
    tax_rate DECIMAL(5, 4) NOT NULL, -- e.g., 0.0825 for 8.25%
    tax_owed_cents BIGINT NOT NULL,
    tax_reserved_cents BIGINT DEFAULT 0, -- amount set aside
    status TEXT DEFAULT 'pending', -- 'pending', 'reserved', 'paid', 'overdue'
    due_date TIMESTAMPTZ,
    paid_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Tax settings per tenant
CREATE TABLE IF NOT EXISTS tax_settings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE UNIQUE,
    business_structure TEXT DEFAULT 'sole_proprietorship', -- 'sole_proprietorship', 'llc', 's_corp', 'c_corp'
    sales_tax_rate DECIMAL(5, 4) DEFAULT 0.0, -- default sales tax rate
    estimated_tax_rate DECIMAL(5, 4) DEFAULT 0.25, -- estimated income tax rate
    self_employment_tax_rate DECIMAL(5, 4) DEFAULT 0.153, -- 15.3% for self-employment
    tax_jurisdiction TEXT, -- state/country for tax rules
    auto_reserve_taxes BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Financial summaries: cached aggregates for quick dashboard access
CREATE TABLE IF NOT EXISTS financial_summaries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    total_revenue_cents BIGINT DEFAULT 0,
    total_expenses_cents BIGINT DEFAULT 0,
    net_income_cents BIGINT DEFAULT 0,
    taxes_reserved_cents BIGINT DEFAULT 0,
    cash_flow_cents BIGINT DEFAULT 0,
    summary_type TEXT DEFAULT 'daily', -- 'daily', 'weekly', 'monthly', 'quarterly', 'yearly'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, period_start, period_end, summary_type)
);

-- Receipt processing queue: for async OCR and categorization
CREATE TABLE IF NOT EXISTS receipt_processing_queue (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    image_url TEXT NOT NULL,
    status TEXT DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed'
    extracted_data JSONB DEFAULT '{}',
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMPTZ
);

-- Enable RLS for all new tables
ALTER TABLE financial_transactions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_financial_transactions ON financial_transactions 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE ledger_entries ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE tax_obligations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tax_obligations ON tax_obligations 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE tax_settings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tax_settings ON tax_settings 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE financial_summaries ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_financial_summaries ON financial_summaries 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE receipt_processing_queue ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_receipt_processing_queue ON receipt_processing_queue 
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_financial_transactions_tenant_occurred 
    ON financial_transactions(tenant_id, occurred_at DESC);
CREATE INDEX IF NOT EXISTS idx_financial_transactions_type 
    ON financial_transactions(tenant_id, transaction_type);
CREATE INDEX IF NOT EXISTS idx_ledger_entries_transaction 
    ON ledger_entries(transaction_id);
CREATE INDEX IF NOT EXISTS idx_tax_obligations_tenant_period 
    ON tax_obligations(tenant_id, period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_financial_summaries_tenant_period 
    ON financial_summaries(tenant_id, period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_receipt_queue_status 
    ON receipt_processing_queue(tenant_id, status);
