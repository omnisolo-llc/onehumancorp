CREATE TABLE IF NOT EXISTS ohc_wallet (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    available_balance_cents BIGINT NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_wallet_tenant ON ohc_wallet(tenant_id);

ALTER TABLE ohc_wallet ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_wallet ON ohc_wallet;
CREATE POLICY tenant_isolation_ohc_wallet
ON ohc_wallet
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ohc_virtual_card (
    id TEXT PRIMARY KEY,
    wallet_id TEXT NOT NULL REFERENCES ohc_wallet(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'ACTIVE', -- ACTIVE, LOCKED, CANCELED
    tokenized_pan TEXT NOT NULL,
    last_four TEXT NOT NULL,
    expiry_month INTEGER NOT NULL,
    expiry_year INTEGER NOT NULL,
    cardholder_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_virtual_card_tenant ON ohc_virtual_card(tenant_id);

ALTER TABLE ohc_virtual_card ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_virtual_card ON ohc_virtual_card;
CREATE POLICY tenant_isolation_ohc_virtual_card
ON ohc_virtual_card
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
