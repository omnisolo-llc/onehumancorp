-- Migration 210: Centralized Inventory Ledger

CREATE TABLE IF NOT EXISTS inventory_levels (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    variant_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    location_id TEXT NOT NULL,
    available_count INT NOT NULL DEFAULT 0,
    committed_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_inventory_levels_tenant_id ON inventory_levels(tenant_id);
CREATE INDEX IF NOT EXISTS idx_inventory_levels_variant_id ON inventory_levels(variant_id);

ALTER TABLE IF EXISTS inventory_levels ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inventory_levels ON inventory_levels;
CREATE POLICY tenant_isolation_inventory_levels
ON inventory_levels
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS inventory_transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    inventory_level_id TEXT REFERENCES inventory_levels(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    quantity_change INT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_inventory_transactions_tenant_id ON inventory_transactions(tenant_id);

ALTER TABLE IF EXISTS inventory_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inventory_transactions ON inventory_transactions;
CREATE POLICY tenant_isolation_inventory_transactions
ON inventory_transactions
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
