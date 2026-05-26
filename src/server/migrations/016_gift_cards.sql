-- Migration 016: Gift Cards and Store Credit Ledger

-- Create gift_cards table
CREATE TABLE IF NOT EXISTS gift_cards (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    code TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL CHECK (type IN ('GIFT_CARD', 'STORE_CREDIT')),
    initial_balance DECIMAL NOT NULL,
    current_balance DECIMAL NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('ACTIVE', 'EXHAUSTED', 'VOID')),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_gift_cards_tenant_id ON gift_cards(tenant_id);
CREATE INDEX IF NOT EXISTS idx_gift_cards_code ON gift_cards(code);

-- Enable RLS on gift_cards
ALTER TABLE gift_cards ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_gift_cards ON gift_cards;
CREATE POLICY tenant_isolation_gift_cards ON gift_cards USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Create ledger_entries table
CREATE TABLE IF NOT EXISTS gift_card_ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    gift_card_id TEXT NOT NULL REFERENCES gift_cards(id) ON DELETE CASCADE,
    amount DECIMAL NOT NULL, -- Negative for redemption, positive for reload/issue
    transaction_ref TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_gift_card_ledger_entries_tenant_id ON gift_card_ledger_entries(tenant_id);
CREATE INDEX IF NOT EXISTS idx_gift_card_ledger_entries_gift_card_id ON gift_card_ledger_entries(gift_card_id);

-- Enable RLS on gift_card_ledger_entries
ALTER TABLE gift_card_ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_gift_card_ledger_entries ON gift_card_ledger_entries;
CREATE POLICY tenant_isolation_gift_card_ledger_entries ON gift_card_ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true));
