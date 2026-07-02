-- Migration to provision Real Estate specific tables
-- Supports: Elena the Property Manager persona

CREATE TABLE IF NOT EXISTS real_estate_properties (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    address TEXT NOT NULL,
    property_type TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS real_estate_units (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    property_id TEXT NOT NULL,
    unit_number TEXT NOT NULL,
    bedrooms INTEGER,
    bathrooms NUMERIC(3, 1),
    square_feet INTEGER,
    rent_amount NUMERIC(10, 2),
    status TEXT NOT NULL DEFAULT 'vacant',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_property FOREIGN KEY(property_id) REFERENCES real_estate_properties(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS real_estate_leases (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    unit_id TEXT NOT NULL,
    tenant_name TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    rent_amount NUMERIC(10, 2) NOT NULL,
    deposit_amount NUMERIC(10, 2),
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_unit FOREIGN KEY(unit_id) REFERENCES real_estate_units(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS maintenance_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    unit_id TEXT NOT NULL,
    reporter_name TEXT NOT NULL,
    issue_description TEXT NOT NULL,
    priority TEXT NOT NULL DEFAULT 'normal',
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_unit_maintenance FOREIGN KEY(unit_id) REFERENCES real_estate_units(id) ON DELETE CASCADE
);

-- Enable RLS for all new tables
ALTER TABLE real_estate_properties ENABLE ROW LEVEL SECURITY;
ALTER TABLE real_estate_units ENABLE ROW LEVEL SECURITY;
ALTER TABLE real_estate_leases ENABLE ROW LEVEL SECURITY;
ALTER TABLE maintenance_requests ENABLE ROW LEVEL SECURITY;

-- Create policies for multi-tenant isolation
CREATE POLICY real_estate_properties_tenant_isolation ON real_estate_properties
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY real_estate_units_tenant_isolation ON real_estate_units
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY real_estate_leases_tenant_isolation ON real_estate_leases
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY maintenance_requests_tenant_isolation ON maintenance_requests
    USING (tenant_id = current_setting('app.current_tenant', true));
