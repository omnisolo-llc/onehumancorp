CREATE TABLE IF NOT EXISTS payment_routing_rules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_service_id TEXT NOT NULL,
    split_percentage DOUBLE PRECISION NOT NULL,
    destination_party_id TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_payment_routing_rules_tenant ON payment_routing_rules(tenant_id);

ALTER TABLE payment_routing_rules ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_payment_routing_rules ON payment_routing_rules USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS transaction_groups (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    reference_type TEXT NOT NULL,
    reference_id TEXT NOT NULL,
    status TEXT DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_transaction_groups_tenant ON transaction_groups(tenant_id);

ALTER TABLE transaction_groups ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_transaction_groups ON transaction_groups USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS invisible_ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    transaction_group_id TEXT NOT NULL,
    entry_type TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    currency TEXT NOT NULL,
    source_party_id TEXT NOT NULL,
    destination_party_id TEXT NOT NULL,
    status TEXT DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (tenant_id, transaction_group_id) REFERENCES transaction_groups(tenant_id, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_invisible_ledger_entries_tenant ON invisible_ledger_entries(tenant_id);

ALTER TABLE invisible_ledger_entries ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_invisible_ledger_entries ON invisible_ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Append-only constraint for invisible_ledger_entries
CREATE OR REPLACE FUNCTION prevent_invisible_ledger_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'invisible_ledger_entries is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_invisible_ledger_update ON invisible_ledger_entries;
CREATE TRIGGER trg_append_only_invisible_ledger_update
BEFORE UPDATE ON invisible_ledger_entries
FOR EACH ROW EXECUTE FUNCTION prevent_invisible_ledger_update_or_delete();

DROP TRIGGER IF EXISTS trg_append_only_invisible_ledger_delete ON invisible_ledger_entries;
CREATE TRIGGER trg_append_only_invisible_ledger_delete
BEFORE DELETE ON invisible_ledger_entries
FOR EACH ROW EXECUTE FUNCTION prevent_invisible_ledger_update_or_delete();
