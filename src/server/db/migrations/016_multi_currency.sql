CREATE TABLE IF NOT EXISTS multi_currency_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    source_currency TEXT NOT NULL,
    target_currency TEXT NOT NULL,
    exchange_rate DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_multi_currency_ledger_tenant
ON multi_currency_ledger(tenant_id, created_at DESC);

ALTER TABLE multi_currency_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_multi_currency_ledger ON multi_currency_ledger;
CREATE POLICY tenant_isolation_multi_currency_ledger
ON multi_currency_ledger
USING (tenant_id = current_setting('app.current_tenant', true));

-- Implement append-only constraint via trigger
CREATE OR REPLACE FUNCTION prevent_multi_currency_ledger_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' AND NEW.status != OLD.status THEN
        -- Allow status updates
        RETURN NEW;
    END IF;
    RAISE EXCEPTION 'multi_currency_ledger is mostly append-only, only status updates allowed';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_multi_currency_ledger_update ON multi_currency_ledger;
CREATE TRIGGER trg_append_only_multi_currency_ledger_update
BEFORE UPDATE ON multi_currency_ledger
FOR EACH ROW EXECUTE FUNCTION prevent_multi_currency_ledger_update_or_delete();

DROP TRIGGER IF EXISTS trg_append_only_multi_currency_ledger_delete ON multi_currency_ledger;
CREATE TRIGGER trg_append_only_multi_currency_ledger_delete
BEFORE DELETE ON multi_currency_ledger
FOR EACH ROW EXECUTE FUNCTION prevent_multi_currency_ledger_update_or_delete();
