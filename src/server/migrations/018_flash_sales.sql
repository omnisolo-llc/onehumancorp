-- Migration 018: Perishable Inventory & Flash Sales
ALTER TABLE products ADD COLUMN IF NOT EXISTS default_ttl_minutes INT DEFAULT 0;
ALTER TABLE products ADD COLUMN IF NOT EXISTS is_perishable BOOLEAN DEFAULT false;

CREATE TABLE IF NOT EXISTS capacity_ledger (
    entry_id TEXT PRIMARY KEY,
    item_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    available_quantity INT DEFAULT 0,
    expiration_time TIMESTAMPTZ,
    status TEXT DEFAULT 'AVAILABLE',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS flash_sale_events (
    event_id TEXT PRIMARY KEY,
    ledger_entry_id TEXT REFERENCES capacity_ledger(entry_id) ON DELETE CASCADE,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    target_audience TEXT,
    discount_amount DECIMAL,
    broadcast_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    status TEXT DEFAULT 'PENDING'
);

ALTER TABLE capacity_ledger ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capacity_ledger ON capacity_ledger USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE flash_sale_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_flash_sale_events ON flash_sale_events USING (tenant_id::text = current_setting('app.current_tenant', true));
