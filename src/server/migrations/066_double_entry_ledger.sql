-- Create double-entry ledger accounts table
CREATE TABLE IF NOT EXISTS ledger_accounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    currency TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, id)
);

CREATE INDEX IF NOT EXISTS idx_ledger_accounts_org_curr ON ledger_accounts(organization_id, currency);

ALTER TABLE ledger_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_accounts ON ledger_accounts;
CREATE POLICY tenant_isolation_ledger_accounts
ON ledger_accounts
USING (tenant_id = current_setting('app.current_tenant', true));


-- Create double-entry ledger transactions table
CREATE TABLE IF NOT EXISTS ledger_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    currency TEXT NOT NULL,
    description TEXT,
    reference_type TEXT,
    reference_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, id)
);

CREATE INDEX IF NOT EXISTS idx_ledger_transactions_org ON ledger_transactions(organization_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ledger_transactions_ref ON ledger_transactions(reference_type, reference_id);

ALTER TABLE ledger_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_transactions ON ledger_transactions;
CREATE POLICY tenant_isolation_ledger_transactions
ON ledger_transactions
USING (tenant_id = current_setting('app.current_tenant', true));


-- Create double-entry ledger entries table
CREATE TABLE IF NOT EXISTS ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL REFERENCES ledger_transactions(id) ON DELETE CASCADE,
    account_id TEXT NOT NULL REFERENCES ledger_accounts(id),
    amount_cents BIGINT NOT NULL, -- Absolute value of the entry
    direction TEXT NOT NULL CHECK (direction IN ('CREDIT', 'DEBIT')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, id)
);

CREATE INDEX IF NOT EXISTS idx_ledger_entries_tx ON ledger_entries(transaction_id);
CREATE INDEX IF NOT EXISTS idx_ledger_entries_account ON ledger_entries(account_id, created_at DESC);

ALTER TABLE ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_entries ON ledger_entries;
CREATE POLICY tenant_isolation_ledger_entries
ON ledger_entries
USING (tenant_id = current_setting('app.current_tenant', true));

-- Append-only constraint via trigger for ledger_transactions
CREATE OR REPLACE FUNCTION prevent_ledger_transaction_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'ledger_transactions is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_ledger_transaction_update ON ledger_transactions;
CREATE TRIGGER trg_append_only_ledger_transaction_update
BEFORE UPDATE ON ledger_transactions
FOR EACH ROW EXECUTE FUNCTION prevent_ledger_transaction_update_or_delete();

DROP TRIGGER IF EXISTS trg_append_only_ledger_transaction_delete ON ledger_transactions;
CREATE TRIGGER trg_append_only_ledger_transaction_delete
BEFORE DELETE ON ledger_transactions
FOR EACH ROW EXECUTE FUNCTION prevent_ledger_transaction_update_or_delete();


-- Append-only constraint via trigger for ledger_entries
CREATE OR REPLACE FUNCTION prevent_ledger_entry_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'ledger_entries is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_ledger_entry_update ON ledger_entries;
CREATE TRIGGER trg_append_only_ledger_entry_update
BEFORE UPDATE ON ledger_entries
FOR EACH ROW EXECUTE FUNCTION prevent_ledger_entry_update_or_delete();

DROP TRIGGER IF EXISTS trg_append_only_ledger_entry_delete ON ledger_entries;
CREATE TRIGGER trg_append_only_ledger_entry_delete
BEFORE DELETE ON ledger_entries
FOR EACH ROW EXECUTE FUNCTION prevent_ledger_entry_update_or_delete();
