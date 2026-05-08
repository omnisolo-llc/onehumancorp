-- 077_add_quotes_table.sql

CREATE TABLE IF NOT EXISTS quotes (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES tenants(tenant_id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE CASCADE,
    amount BIGINT NOT NULL, -- Stored in cents
    status TEXT NOT NULL CHECK (status IN ('draft', 'approved', 'paid')),
    booking_id UUID REFERENCES bookings(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_quotes_tenant ON quotes(tenant_id);

ALTER TABLE quotes ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_quotes ON quotes
    USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY system_isolation_quotes ON quotes
    USING (current_setting('app.current_tenant', true) = 'system');
