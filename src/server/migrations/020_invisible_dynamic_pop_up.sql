-- Migration 020: Invisible Dynamic Pop-Up & Geo-Commerce Orchestration Engine

-- Add popup_session table
CREATE TABLE IF NOT EXISTS popup_sessions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    location_name TEXT NOT NULL,
    geo_coordinates JSONB,
    status TEXT DEFAULT 'active', -- 'active', 'completed', 'cancelled'
    tax_nexus TEXT,
    started_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE popup_sessions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_popup_sessions ON popup_sessions USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Add inventory_allocations table
CREATE TABLE IF NOT EXISTS inventory_allocations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    popup_session_id TEXT REFERENCES popup_sessions(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    allocated_quantity INT NOT NULL,
    sold_quantity INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE inventory_allocations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_inventory_allocations ON inventory_allocations USING (tenant_id::text = current_setting('app.current_tenant', true));
