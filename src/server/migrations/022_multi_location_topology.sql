-- Migration 022: Multi-Location Topology Engine

-- Create location_nodes table
CREATE TABLE IF NOT EXISTS location_nodes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    address TEXT,
    geo_coordinate TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on location_nodes
ALTER TABLE location_nodes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_location_nodes ON location_nodes USING (tenant_id::text = current_setting('app.current_tenant', true));


-- Create inventory_ledgers table
CREATE TABLE IF NOT EXISTS inventory_ledgers (
    id TEXT PRIMARY KEY,
    node_id TEXT REFERENCES location_nodes(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    quantity_on_hand INT DEFAULT 0,
    local_price_override DECIMAL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on inventory_ledgers
ALTER TABLE inventory_ledgers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_inventory_ledgers ON inventory_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true));


-- Create staff_rosters table
CREATE TABLE IF NOT EXISTS staff_rosters (
    id TEXT PRIMARY KEY,
    node_id TEXT REFERENCES location_nodes(id) ON DELETE CASCADE,
    user_id TEXT REFERENCES users(id) ON DELETE CASCADE,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on staff_rosters
ALTER TABLE staff_rosters ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_staff_rosters ON staff_rosters USING (tenant_id::text = current_setting('app.current_tenant', true));


-- Create local_tax_nexuses table
CREATE TABLE IF NOT EXISTS local_tax_nexuses (
    id TEXT PRIMARY KEY,
    node_id TEXT REFERENCES location_nodes(id) ON DELETE CASCADE,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    tax_rate DECIMAL NOT NULL,
    tax_region TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on local_tax_nexuses
ALTER TABLE local_tax_nexuses ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_local_tax_nexuses ON local_tax_nexuses USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Add shared tasks list support for RLS policy update
DO $$
BEGIN
    IF to_regclass('location_nodes') IS NOT NULL THEN
        ALTER TABLE IF EXISTS location_nodes FORCE ROW LEVEL SECURITY;
    END IF;
    IF to_regclass('inventory_ledgers') IS NOT NULL THEN
        ALTER TABLE IF EXISTS inventory_ledgers FORCE ROW LEVEL SECURITY;
    END IF;
    IF to_regclass('staff_rosters') IS NOT NULL THEN
        ALTER TABLE IF EXISTS staff_rosters FORCE ROW LEVEL SECURITY;
    END IF;
    IF to_regclass('local_tax_nexuses') IS NOT NULL THEN
        ALTER TABLE IF EXISTS local_tax_nexuses FORCE ROW LEVEL SECURITY;
    END IF;
END
$$;