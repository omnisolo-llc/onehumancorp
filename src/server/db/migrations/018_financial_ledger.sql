-- Architecture: Autonomous Unified Ledger and Multi-Currency Settlement Engine
-- GitHub Issue #22745

CREATE TABLE IF NOT EXISTS ledger_accounts (
    account_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    currency TEXT NOT NULL,
    balance NUMERIC(19, 4) NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ledger_accounts_tenant
ON ledger_accounts(tenant_id);

ALTER TABLE ledger_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_accounts ON ledger_accounts;
CREATE POLICY tenant_isolation_ledger_accounts
ON ledger_accounts
USING (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ledger_transactions (
    tx_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    amount NUMERIC(19, 4) NOT NULL,
    currency TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ledger_transactions_tenant
ON ledger_transactions(tenant_id);

ALTER TABLE ledger_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_transactions ON ledger_transactions;
CREATE POLICY tenant_isolation_ledger_transactions
ON ledger_transactions
USING (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS double_entry_ledger (
    entry_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    tx_id TEXT NOT NULL REFERENCES ledger_transactions(tx_id),
    account_id TEXT NOT NULL REFERENCES ledger_accounts(account_id),
    direction TEXT NOT NULL, -- 'CREDIT' or 'DEBIT'
    amount NUMERIC(19, 4) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_double_entry_ledger_tenant
ON double_entry_ledger(tenant_id);

ALTER TABLE double_entry_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_double_entry_ledger ON double_entry_ledger;
CREATE POLICY tenant_isolation_double_entry_ledger
ON double_entry_ledger
USING (tenant_id = current_setting('app.current_tenant', true));

-- Append only constraint for transactions and entries
CREATE OR REPLACE FUNCTION prevent_update_or_delete_on_immutable_ledger()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'This table is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_ledger_tx_update ON ledger_transactions;
CREATE TRIGGER trg_append_only_ledger_tx_update
BEFORE UPDATE ON ledger_transactions
FOR EACH ROW EXECUTE FUNCTION prevent_update_or_delete_on_immutable_ledger();

DROP TRIGGER IF EXISTS trg_append_only_ledger_tx_delete ON ledger_transactions;
CREATE TRIGGER trg_append_only_ledger_tx_delete
BEFORE DELETE ON ledger_transactions
FOR EACH ROW EXECUTE FUNCTION prevent_update_or_delete_on_immutable_ledger();

DROP TRIGGER IF EXISTS trg_append_only_ledger_entry_update ON double_entry_ledger;
CREATE TRIGGER trg_append_only_ledger_entry_update
BEFORE UPDATE ON double_entry_ledger
FOR EACH ROW EXECUTE FUNCTION prevent_update_or_delete_on_immutable_ledger();

DROP TRIGGER IF EXISTS trg_append_only_ledger_entry_delete ON double_entry_ledger;
CREATE TRIGGER trg_append_only_ledger_entry_delete
BEFORE DELETE ON double_entry_ledger
FOR EACH ROW EXECUTE FUNCTION prevent_update_or_delete_on_immutable_ledger();
