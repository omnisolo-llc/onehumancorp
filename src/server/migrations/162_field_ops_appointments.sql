-- +goose Up

CREATE TABLE IF NOT EXISTS job_templates (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS appointments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    job_template_id TEXT REFERENCES job_templates(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'Requested',
    scheduled_start_time TIMESTAMPTZ,
    scheduled_end_time TIMESTAMPTZ,
    location_address TEXT,
    location_lat DOUBLE PRECISION,
    location_lng DOUBLE PRECISION,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS service_routes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    agent_id TEXT,
    route_date DATE NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS route_stops (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    route_id TEXT REFERENCES service_routes(id) ON DELETE CASCADE,
    appointment_id TEXT REFERENCES appointments(id) ON DELETE CASCADE,
    stop_order INTEGER NOT NULL,
    estimated_arrival TIMESTAMPTZ,
    estimated_departure TIMESTAMPTZ,
    travel_time_minutes INTEGER,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE job_templates ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_job_templates ON job_templates
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE appointments ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_appointments ON appointments
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE service_routes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_service_routes ON service_routes
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE route_stops ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_route_stops ON route_stops
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_route_stops ON route_stops;
ALTER TABLE route_stops DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS route_stops CASCADE;

DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
ALTER TABLE service_routes DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS service_routes CASCADE;

DROP POLICY IF EXISTS tenant_isolation_appointments ON appointments;
ALTER TABLE appointments DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS appointments CASCADE;

DROP POLICY IF EXISTS tenant_isolation_job_templates ON job_templates;
ALTER TABLE job_templates DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS job_templates CASCADE;
