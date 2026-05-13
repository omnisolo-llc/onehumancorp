-- 079_bookings.sql

CREATE TABLE IF NOT EXISTS services (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    duration_minutes INT NOT NULL,
    price_cents BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_services_tenant ON services(tenant_id);
ALTER TABLE services ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_services ON services
    USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY system_isolation_services ON services
    USING (current_setting('app.current_tenant', true) = 'system');


CREATE TABLE IF NOT EXISTS availability_slots (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    service_id UUID REFERENCES services(id) ON DELETE CASCADE,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    is_booked BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_availability_slots_tenant ON availability_slots(tenant_id);
CREATE INDEX idx_availability_slots_service ON availability_slots(service_id);
ALTER TABLE availability_slots ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_availability_slots ON availability_slots
    USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY system_isolation_availability_slots ON availability_slots
    USING (current_setting('app.current_tenant', true) = 'system');


CREATE TABLE IF NOT EXISTS bookings (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    service_id UUID REFERENCES services(id) ON DELETE RESTRICT,
    slot_id UUID REFERENCES availability_slots(id) ON DELETE RESTRICT,
    customer_id UUID NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'confirmed', 'cancelled', 'completed')),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bookings_tenant ON bookings(tenant_id);
CREATE INDEX idx_bookings_service ON bookings(service_id);
ALTER TABLE bookings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_bookings ON bookings
    USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY system_isolation_bookings ON bookings
    USING (current_setting('app.current_tenant', true) = 'system');
