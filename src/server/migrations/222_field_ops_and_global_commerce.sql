-- Keep PostgreSQL's active SQLx migration stream aligned with the field-ops
-- and global-commerce handlers. These tables/columns existed in the legacy
-- schema tree but were not present in the migrations executed at startup.

CREATE TABLE IF NOT EXISTS job_locations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_route_id TEXT NOT NULL REFERENCES service_routes(id) ON DELETE CASCADE,
    appointment_id TEXT NOT NULL REFERENCES appointments(id) ON DELETE CASCADE,
    sequence_order INTEGER NOT NULL,
    estimated_travel_time_mins INTEGER,
    distance_to_next_km DOUBLE PRECISION,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(service_route_id, sequence_order)
);

CREATE INDEX IF NOT EXISTS idx_job_locations_tenant_id
    ON job_locations (tenant_id);
CREATE INDEX IF NOT EXISTS idx_job_locations_route
    ON job_locations (tenant_id, service_route_id, sequence_order);

ALTER TABLE tenants
    ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD';
ALTER TABLE tenants
    ADD COLUMN IF NOT EXISTS enabled_currencies JSONB DEFAULT '["USD"]'::jsonb;

UPDATE tenants
SET base_currency = COALESCE(base_currency, 'USD'),
    enabled_currencies = COALESCE(enabled_currencies, '["USD"]'::jsonb)
WHERE base_currency IS NULL OR enabled_currencies IS NULL;

ALTER TABLE job_locations ENABLE ROW LEVEL SECURITY;
ALTER TABLE job_locations FORCE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'job_locations'
          AND policyname = 'tenant_isolation_job_locations'
    ) THEN
        CREATE POLICY tenant_isolation_job_locations
            ON job_locations
            USING (tenant_id = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
    END IF;
END
$$;
