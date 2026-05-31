-- Create raw_materials and purchase_orders tables

CREATE TABLE IF NOT EXISTS raw_materials (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    current_quantity INTEGER DEFAULT 0,
    reorder_threshold INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS purchase_orders (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    vendor_id TEXT NOT NULL,
    status TEXT NOT NULL,
    total_cost DOUBLE PRECISION,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE raw_materials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_raw_materials ON raw_materials;
CREATE POLICY tenant_isolation_raw_materials ON raw_materials USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE purchase_orders ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_purchase_orders ON purchase_orders;
CREATE POLICY tenant_isolation_purchase_orders ON purchase_orders USING (tenant_id::text = current_setting('app.current_tenant', true));
