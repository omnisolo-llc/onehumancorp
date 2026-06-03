-- Gift Card and Store Credit Ledger
-- Implements offline-first capable, append-only ledger

CREATE TABLE IF NOT EXISTS ohc_gift_cards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    code TEXT NOT NULL,
    card_type TEXT NOT NULL, -- 'GIFT_CARD', 'STORE_CREDIT'
    status TEXT NOT NULL DEFAULT 'ACTIVE', -- 'ACTIVE', 'EXHAUSTED', 'VOID'
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_ohc_gift_cards_code ON ohc_gift_cards(tenant_id, code);

ALTER TABLE ohc_gift_cards ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_gift_cards ON ohc_gift_cards;
CREATE POLICY tenant_isolation_ohc_gift_cards
ON ohc_gift_cards
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ohc_gift_card_ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    gift_card_id TEXT NOT NULL REFERENCES ohc_gift_cards(id),
    amount BIGINT NOT NULL, -- Using BIGINT for cents. Negative for redemption, positive for issue/reload
    transaction_ref TEXT,
    is_offline_sync BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_gift_card_ledger_entries_card ON ohc_gift_card_ledger_entries(tenant_id, gift_card_id, created_at DESC);

ALTER TABLE ohc_gift_card_ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_gift_card_ledger_entries ON ohc_gift_card_ledger_entries;
CREATE POLICY tenant_isolation_ohc_gift_card_ledger_entries
ON ohc_gift_card_ledger_entries
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Note: Append-only triggers are managed in application code logic or explicit triggers,
-- but skipping the PL/pgSQL function here for wider compatibility with migration testing tools
-- unless explicitly required by existing conventions. We will implement it since the prompt requests it.

CREATE OR REPLACE FUNCTION prevent_gift_card_ledger_update_or_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'ohc_gift_card_ledger_entries is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_append_only_gift_card_ledger_update ON ohc_gift_card_ledger_entries;
CREATE TRIGGER trg_append_only_gift_card_ledger_update
BEFORE UPDATE ON ohc_gift_card_ledger_entries
FOR EACH ROW EXECUTE FUNCTION prevent_gift_card_ledger_update_or_delete();

DROP TRIGGER IF EXISTS trg_append_only_gift_card_ledger_delete ON ohc_gift_card_ledger_entries;
CREATE TRIGGER trg_append_only_gift_card_ledger_delete
BEFORE DELETE ON ohc_gift_card_ledger_entries
FOR EACH ROW EXECUTE FUNCTION prevent_gift_card_ledger_update_or_delete();
