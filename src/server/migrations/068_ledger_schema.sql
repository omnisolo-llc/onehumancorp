
-- Ledger Schemas
CREATE TABLE IF NOT EXISTS ledger_accounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    currency TEXT NOT NULL,
    balance DECIMAL(15,2) DEFAULT 0.0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ledger_accounts_tenant ON ledger_accounts(tenant_id);

ALTER TABLE ledger_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_accounts ON ledger_accounts;
CREATE POLICY tenant_isolation_ledger_accounts ON ledger_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ledger_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency TEXT NOT NULL,
    description TEXT,
    reference_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ledger_transactions_tenant ON ledger_transactions(tenant_id);

ALTER TABLE ledger_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_transactions ON ledger_transactions;
CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL REFERENCES ledger_transactions(id) ON DELETE CASCADE,
    account_id TEXT NOT NULL REFERENCES ledger_accounts(id) ON DELETE RESTRICT,
    direction TEXT NOT NULL, -- 'CREDIT' or 'DEBIT'
    amount DECIMAL(15,2) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant ON ledger_entries(tenant_id);
CREATE INDEX IF NOT EXISTS idx_ledger_entries_tx ON ledger_entries(transaction_id);

ALTER TABLE ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_entries ON ledger_entries;
CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
