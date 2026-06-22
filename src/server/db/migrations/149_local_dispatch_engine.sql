-- 149_local_dispatch_engine.sql

-- Route table
CREATE TABLE IF NOT EXISTS local_dispatch_routes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    vehicle_id UUID,
    start_time TIMESTAMPTZ,
    end_time TIMESTAMPTZ,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE local_dispatch_routes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation ON local_dispatch_routes
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant')::uuid);

-- Stops table
CREATE TABLE IF NOT EXISTS local_dispatch_stops (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    route_id UUID NOT NULL REFERENCES local_dispatch_routes(id) ON DELETE CASCADE,
    location_id UUID NOT NULL,
    sequence_number INTEGER NOT NULL,
    estimated_arrival TIMESTAMPTZ,
    actual_arrival TIMESTAMPTZ,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE local_dispatch_stops ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation ON local_dispatch_stops
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant')::uuid);

-- Courier Dispatches
CREATE TABLE IF NOT EXISTS courier_dispatches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    courier_name TEXT,
    courier_phone TEXT,
    current_location_lat DOUBLE PRECISION,
    current_location_lng DOUBLE PRECISION,
    status TEXT NOT NULL DEFAULT 'ASSIGNED',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE courier_dispatches ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation ON courier_dispatches
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant')::uuid);
