-- +goose Up
CREATE TABLE IF NOT EXISTS bookings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    service_id TEXT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    resource_id TEXT REFERENCES booking_resources(id) ON DELETE SET NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'pending_payment', 'confirmed', 'completed', 'cancelled')),
    payment_intent_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_bookings_tenant_id ON bookings(tenant_id);
CREATE INDEX IF NOT EXISTS idx_bookings_customer_id ON bookings(customer_id);

ALTER TABLE bookings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
CREATE POLICY tenant_isolation_bookings
ON bookings
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
DROP TABLE IF EXISTS bookings CASCADE;
