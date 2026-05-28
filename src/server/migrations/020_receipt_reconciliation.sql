-- Migration 020: Zero-Touch Expense & Receipt Reconciliation Engine

CREATE TABLE IF NOT EXISTS receipts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    storage_url TEXT NOT NULL,
    status TEXT DEFAULT 'pending_processing',
    extracted_vendor TEXT,
    extracted_date TIMESTAMPTZ,
    extracted_amount DECIMAL,
    extracted_tax DECIMAL,
    confidence_score DECIMAL,
    raw_ocr_data JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE receipts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_receipts ON receipts USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS bank_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    account_id TEXT NOT NULL,
    transaction_date TIMESTAMPTZ NOT NULL,
    amount DECIMAL NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'unreconciled',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE bank_transactions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_bank_transactions ON bank_transactions USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    transaction_id TEXT REFERENCES bank_transactions(id) ON DELETE SET NULL,
    receipt_id TEXT REFERENCES receipts(id) ON DELETE SET NULL,
    amount DECIMAL NOT NULL,
    category TEXT NOT NULL,
    description TEXT,
    entry_type TEXT NOT NULL, -- 'expense', 'income', etc.
    status TEXT DEFAULT 'auto_matched', -- 'auto_matched', 'pending_review', 'approved'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE ledger_entries ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true));
