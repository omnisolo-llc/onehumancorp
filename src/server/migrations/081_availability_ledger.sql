CREATE TABLE IF NOT EXISTS availability_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    status TEXT NOT NULL DEFAULT 'AVAILABLE', -- AVAILABLE, BLOCKED, TENTATIVE, BOOKED
    booking_id TEXT REFERENCES bookings(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS travel_buffers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    booking_id TEXT REFERENCES bookings(id) ON DELETE CASCADE,
    pre_buffer_minutes INTEGER NOT NULL DEFAULT 0,
    post_buffer_minutes INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE availability_ledger ENABLE ROW LEVEL SECURITY;
CREATE POLICY availability_ledger_tenant_isolation ON availability_ledger
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE travel_buffers ENABLE ROW LEVEL SECURITY;
CREATE POLICY travel_buffers_tenant_isolation ON travel_buffers
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS idx_availability_ledger_tenant_time ON availability_ledger(tenant_id, start_time, end_time);
CREATE INDEX IF NOT EXISTS idx_travel_buffers_tenant_booking ON travel_buffers(tenant_id, booking_id);
