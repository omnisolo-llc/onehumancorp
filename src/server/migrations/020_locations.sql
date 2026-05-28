-- +goose Up
-- Migration 020: Location Graph and Inventory Ledger

CREATE TABLE IF NOT EXISTS location_nodes (
    node_id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    geo_location TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS inventory_ledgers (
    node_id TEXT REFERENCES location_nodes(node_id) ON DELETE CASCADE,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    available_qty INT DEFAULT 0,
    reserved_qty INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, node_id, product_id)
);

-- RLS Policies
ALTER TABLE location_nodes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_location_nodes ON location_nodes USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE inventory_ledgers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_inventory_ledgers ON inventory_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
-- Reverse Migration 020
DROP POLICY IF EXISTS tenant_isolation_inventory_ledgers ON inventory_ledgers;
ALTER TABLE inventory_ledgers DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS inventory_ledgers CASCADE;

DROP POLICY IF EXISTS tenant_isolation_location_nodes ON location_nodes;
ALTER TABLE location_nodes DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS location_nodes CASCADE;
