-- Create accounts table
CREATE TABLE IF NOT EXISTS ledger_accounts (
    tenant_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    currency TEXT NOT NULL,
    balance DOUBLE PRECISION DEFAULT 0.0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, account_id)
);

CREATE INDEX IF NOT EXISTS idx_ledger_accounts_tenant ON ledger_accounts(tenant_id);

ALTER TABLE ledger_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_accounts ON ledger_accounts;
CREATE POLICY tenant_isolation_ledger_accounts ON ledger_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- Create transactions table
CREATE TABLE IF NOT EXISTS ledger_transactions (
    tenant_id TEXT NOT NULL,
    tx_id TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    currency TEXT NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, tx_id)
);

CREATE INDEX IF NOT EXISTS idx_ledger_transactions_tenant ON ledger_transactions(tenant_id);

ALTER TABLE ledger_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_transactions ON ledger_transactions;
CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- Create entries table
CREATE TABLE IF NOT EXISTS ledger_entries (
    tenant_id TEXT NOT NULL,
    entry_id TEXT NOT NULL,
    tx_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    direction TEXT NOT NULL, -- "CREDIT" or "DEBIT"
    amount DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, entry_id),
    FOREIGN KEY (tenant_id, tx_id) REFERENCES ledger_transactions(tenant_id, tx_id),
    FOREIGN KEY (tenant_id, account_id) REFERENCES ledger_accounts(tenant_id, account_id)
);

CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_tx ON ledger_entries(tenant_id, tx_id);
CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_account ON ledger_entries(tenant_id, account_id);

ALTER TABLE ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_entries ON ledger_entries;
CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
