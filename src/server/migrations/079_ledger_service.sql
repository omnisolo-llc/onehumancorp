-- +goose Up
-- Migration 079: Autonomous Unified Ledger Service

CREATE TABLE IF NOT EXISTS accounts (
    tenant_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    currency TEXT NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, account_id)
);

CREATE TABLE IF NOT EXISTS transactions (
    tenant_id TEXT NOT NULL,
    tx_id TEXT NOT NULL,
    amount BIGINT NOT NULL,
    currency TEXT NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, tx_id)
);

CREATE TABLE IF NOT EXISTS entries (
    tenant_id TEXT NOT NULL,
    entry_id TEXT NOT NULL,
    tx_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    direction TEXT NOT NULL,
    amount BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, entry_id)
);

CREATE INDEX IF NOT EXISTS idx_accounts_tenant ON accounts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_transactions_tenant ON transactions(tenant_id);
CREATE INDEX IF NOT EXISTS idx_entries_tenant ON entries(tenant_id);
CREATE INDEX IF NOT EXISTS idx_entries_tx ON entries(tenant_id, tx_id);

ALTER TABLE accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_accounts ON accounts;
CREATE POLICY tenant_isolation_accounts ON accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_transactions ON transactions;
CREATE POLICY tenant_isolation_transactions ON transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_entries ON entries;
CREATE POLICY tenant_isolation_entries ON entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_entries ON entries;
ALTER TABLE entries DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS entries;

DROP POLICY IF EXISTS tenant_isolation_transactions ON transactions;
ALTER TABLE transactions DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS transactions;

DROP POLICY IF EXISTS tenant_isolation_accounts ON accounts;
ALTER TABLE accounts DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS accounts;
