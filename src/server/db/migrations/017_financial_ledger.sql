CREATE TABLE IF NOT EXISTS accounts (
    tenant_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    currency TEXT NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, account_id)
);

CREATE TABLE IF NOT EXISTS transactions (
    tenant_id TEXT NOT NULL,
    tx_id TEXT NOT NULL,
    amount BIGINT NOT NULL,
    currency TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, tx_id)
);

CREATE TABLE IF NOT EXISTS entries (
    tenant_id TEXT NOT NULL,
    entry_id TEXT NOT NULL,
    tx_id TEXT NOT NULL,
    account_id TEXT NOT NULL,
    direction TEXT NOT NULL,
    amount BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, entry_id),
    FOREIGN KEY (tenant_id, tx_id) REFERENCES transactions(tenant_id, tx_id),
    FOREIGN KEY (tenant_id, account_id) REFERENCES accounts(tenant_id, account_id)
);

ALTER TABLE accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_accounts ON accounts;
CREATE POLICY tenant_isolation_accounts ON accounts USING (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_transactions ON transactions;
CREATE POLICY tenant_isolation_transactions ON transactions USING (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_entries ON entries;
CREATE POLICY tenant_isolation_entries ON entries USING (tenant_id = current_setting('app.current_tenant', true));
