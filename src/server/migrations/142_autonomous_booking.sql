CREATE TABLE IF NOT EXISTS resources (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    type TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS availability_blocks (
    id TEXT PRIMARY KEY,
    resource_id TEXT REFERENCES resources(id) ON DELETE CASCADE,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    service_id TEXT REFERENCES services(id) ON DELETE CASCADE,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    is_recurring BOOLEAN DEFAULT false,
    is_available BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE resources ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_resources ON resources USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE availability_blocks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_availability_blocks ON availability_blocks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name='bookings' AND column_name='resource_id') THEN
        ALTER TABLE bookings ADD COLUMN resource_id TEXT REFERENCES resources(id) ON DELETE SET NULL;
    END IF;
END $$;
