-- Migration: Invisible Multi-Party Split Payments & Consignment Ledger

CREATE TABLE IF NOT EXISTS multi_party_splits (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    resource_type TEXT NOT NULL, -- e.g., "invoice", "product"
    resource_id TEXT NOT NULL,
    partner_id TEXT NOT NULL,
    split_percentage DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_multi_party_splits_tenant ON multi_party_splits(tenant_id);
CREATE INDEX IF NOT EXISTS idx_multi_party_splits_resource ON multi_party_splits(tenant_id, resource_type, resource_id);

ALTER TABLE multi_party_splits ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_multi_party_splits ON multi_party_splits USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS multi_party_split_ledgers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    partner_id TEXT NOT NULL,
    payment_event_id TEXT NOT NULL,
    source_resource_type TEXT NOT NULL,
    source_resource_id TEXT NOT NULL,
    total_amount DOUBLE PRECISION NOT NULL,
    partner_amount DOUBLE PRECISION NOT NULL,
    owner_amount DOUBLE PRECISION NOT NULL,
    status TEXT DEFAULT 'PENDING_PAYOUT', -- PENDING_PAYOUT, PAID_OUT
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_multi_party_split_ledgers_tenant ON multi_party_split_ledgers(tenant_id);

ALTER TABLE multi_party_split_ledgers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
