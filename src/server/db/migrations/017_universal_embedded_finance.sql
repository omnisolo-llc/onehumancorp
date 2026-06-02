-- Universal Embedded Finance & AI Taxation Ledger
-- Derived from research report: [architecture] Universal Embedded Finance & AI Taxation Ledger

CREATE TABLE IF NOT EXISTS ohc_double_entry_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    account_type TEXT NOT NULL, -- e.g., 'CASH', 'TAX_VAULT'
    amount DECIMAL(19,4) NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_double_entry_ledger_tenant
ON ohc_double_entry_ledger(tenant_id, created_at DESC);

ALTER TABLE ohc_double_entry_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_double_entry_ledger ON ohc_double_entry_ledger;
CREATE POLICY tenant_isolation_ohc_double_entry_ledger
ON ohc_double_entry_ledger
USING (tenant_id = current_setting('app.current_tenant', true));

-- Append-only constraint via trigger
CREATE OR REPLACE FUNCTION prevent_double_entry_ledger_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'ohc_double_entry_ledger is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_double_entry_ledger_update ON ohc_double_entry_ledger;
CREATE TRIGGER trg_append_only_double_entry_ledger_update
BEFORE UPDATE ON ohc_double_entry_ledger
FOR EACH ROW EXECUTE FUNCTION prevent_double_entry_ledger_update_or_delete();

DROP TRIGGER IF EXISTS trg_append_only_double_entry_ledger_delete ON ohc_double_entry_ledger;
CREATE TRIGGER trg_append_only_double_entry_ledger_delete
BEFORE DELETE ON ohc_double_entry_ledger
FOR EACH ROW EXECUTE FUNCTION prevent_double_entry_ledger_update_or_delete();


CREATE TABLE IF NOT EXISTS ohc_virtual_wallets (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    wallet_type TEXT NOT NULL, -- e.g., 'MAIN_BALANCE', 'TAX_VAULT'
    balance DECIMAL(19,4) NOT NULL DEFAULT 0.0000,
    currency TEXT NOT NULL DEFAULT 'USD',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_wallet_per_tenant UNIQUE (tenant_id, wallet_type)
);

CREATE INDEX IF NOT EXISTS idx_ohc_virtual_wallets_tenant
ON ohc_virtual_wallets(tenant_id);

ALTER TABLE ohc_virtual_wallets ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_virtual_wallets ON ohc_virtual_wallets;
CREATE POLICY tenant_isolation_ohc_virtual_wallets
ON ohc_virtual_wallets
USING (tenant_id = current_setting('app.current_tenant', true));
