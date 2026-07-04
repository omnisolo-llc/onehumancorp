CREATE TABLE IF NOT EXISTS booking_resources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS availability_blocks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    resource_id UUID REFERENCES booking_resources(id) ON DELETE CASCADE,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    is_recurring BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name='bookings' AND column_name='resource_id') THEN
        -- Alter table only if column doesn't exist
        BEGIN
           ALTER TABLE bookings ADD COLUMN resource_id UUID REFERENCES booking_resources(id);
        EXCEPTION
           WHEN duplicate_column THEN NULL;
        END;
    END IF;
END $$;


-- RLS Policies
ALTER TABLE booking_resources ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_resources ON booking_resources;
CREATE POLICY tenant_isolation_booking_resources ON booking_resources USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE availability_blocks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_blocks ON availability_blocks;
CREATE POLICY tenant_isolation_availability_blocks ON availability_blocks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
