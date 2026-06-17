-- +goose Up
-- Agentic Field Service Dispatch & Mobile Estimating Engine

CREATE TABLE IF NOT EXISTS field_service_jobs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    booking_id TEXT REFERENCES bookings(id) ON DELETE SET NULL,
    status TEXT NOT NULL CHECK (status IN ('draft', 'scheduled', 'en_route', 'in_progress', 'completed', 'cancelled')),
    description TEXT,
    estimated_duration_mins INTEGER NOT NULL,
    location_address TEXT NOT NULL,
    location_lat DOUBLE PRECISION,
    location_lng DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_field_service_jobs_tenant_id ON field_service_jobs(tenant_id);

ALTER TABLE field_service_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_field_service_jobs ON field_service_jobs;
CREATE POLICY tenant_isolation_field_service_jobs
ON field_service_jobs
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS route_itineraries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    staff_profile_id TEXT NOT NULL REFERENCES staff_profiles(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('draft', 'optimized', 'active', 'completed')),
    start_location_lat DOUBLE PRECISION,
    start_location_lng DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_route_itineraries_tenant_id ON route_itineraries(tenant_id);
CREATE INDEX IF NOT EXISTS idx_route_itineraries_staff_date ON route_itineraries(staff_profile_id, date);

ALTER TABLE route_itineraries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_route_itineraries ON route_itineraries;
CREATE POLICY tenant_isolation_route_itineraries
ON route_itineraries
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS service_stops (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    job_id TEXT NOT NULL REFERENCES field_service_jobs(id) ON DELETE CASCADE,
    route_itinerary_id TEXT NOT NULL REFERENCES route_itineraries(id) ON DELETE CASCADE,
    sequence_order INTEGER NOT NULL,
    estimated_arrival_time TIMESTAMPTZ,
    actual_arrival_time TIMESTAMPTZ,
    status TEXT NOT NULL CHECK (status IN ('pending', 'arrived', 'completed', 'skipped')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_service_stops_tenant_id ON service_stops(tenant_id);
CREATE INDEX IF NOT EXISTS idx_service_stops_itinerary ON service_stops(route_itinerary_id, sequence_order);

ALTER TABLE service_stops ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_stops ON service_stops;
CREATE POLICY tenant_isolation_service_stops
ON service_stops
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS job_estimates (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    job_id TEXT NOT NULL REFERENCES field_service_jobs(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    total_amount_cents BIGINT NOT NULL,
    labor_amount_cents BIGINT NOT NULL,
    materials_amount_cents BIGINT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('draft', 'sent', 'approved', 'rejected')),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_job_estimates_tenant_id ON job_estimates(tenant_id);

ALTER TABLE job_estimates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_job_estimates ON job_estimates;
CREATE POLICY tenant_isolation_job_estimates
ON job_estimates
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Add travel_time_mins padding to bookings if not exists
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS travel_time_mins INTEGER DEFAULT 0;

-- +goose Down
ALTER TABLE bookings DROP COLUMN IF EXISTS travel_time_mins;
DROP POLICY IF EXISTS tenant_isolation_job_estimates ON job_estimates;
DROP TABLE IF EXISTS job_estimates CASCADE;
DROP POLICY IF EXISTS tenant_isolation_route_itineraries ON route_itineraries;
DROP TABLE IF EXISTS route_itineraries CASCADE;
DROP POLICY IF EXISTS tenant_isolation_service_stops ON service_stops;
DROP TABLE IF EXISTS service_stops CASCADE;
DROP POLICY IF EXISTS tenant_isolation_field_service_jobs ON field_service_jobs;
DROP TABLE IF EXISTS field_service_jobs CASCADE;
