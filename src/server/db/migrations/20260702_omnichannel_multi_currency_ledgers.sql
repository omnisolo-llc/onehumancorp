CREATE TABLE IF NOT EXISTS ledger_account (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    currency TEXT NOT NULL,
    current_balance DECIMAL NOT NULL DEFAULT 0.0,
    last_synced TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE ledger_account ENABLE ROW LEVEL SECURITY;
CREATE POLICY ledger_account_tenant_isolation_policy ON ledger_account FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS ledger_entry (
    id UUID PRIMARY KEY,
    ledger_account_id UUID NOT NULL REFERENCES ledger_account(id),
    amount DECIMAL NOT NULL,
    type TEXT NOT NULL,
    idempotency_key TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE ledger_entry ENABLE ROW LEVEL SECURITY;
CREATE POLICY ledger_entry_tenant_isolation_policy ON ledger_entry FOR ALL USING (
    EXISTS (
        SELECT 1 FROM ledger_account WHERE ledger_account.id = ledger_entry.ledger_account_id AND ledger_account.tenant_id = current_setting('app.current_tenant_id', true)::uuid
    )
);

-- Implement append-only constraint via trigger for ledger_entry
CREATE OR REPLACE FUNCTION prevent_ledger_entry_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'ledger_entry is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_ledger_entry_update ON ledger_entry;
CREATE TRIGGER trg_append_only_ledger_entry_update
BEFORE UPDATE ON ledger_entry
FOR EACH ROW EXECUTE FUNCTION prevent_ledger_entry_update_or_delete();

DROP TRIGGER IF EXISTS trg_append_only_ledger_entry_delete ON ledger_entry;
CREATE TRIGGER trg_append_only_ledger_entry_delete
BEFORE DELETE ON ledger_entry
FOR EACH ROW EXECUTE FUNCTION prevent_ledger_entry_update_or_delete();
