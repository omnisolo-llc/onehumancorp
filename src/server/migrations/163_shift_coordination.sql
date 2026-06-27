-- +goose Up
CREATE TABLE IF NOT EXISTS shifts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    staff_id TEXT NOT NULL,
    role TEXT NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    status TEXT NOT NULL DEFAULT 'scheduled',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_shifts_tenant_id ON shifts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_shifts_staff_id ON shifts(staff_id);

ALTER TABLE shifts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shifts ON shifts;
CREATE POLICY tenant_isolation_shifts
ON shifts
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS staff_availability (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    staff_id TEXT NOT NULL,
    day_of_week INTEGER NOT NULL, -- 0-6 (Sun-Sat)
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_staff_availability_tenant_id ON staff_availability(tenant_id);

ALTER TABLE staff_availability ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_availability ON staff_availability;
CREATE POLICY tenant_isolation_staff_availability
ON staff_availability
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_staff_availability ON staff_availability;
DROP TABLE IF EXISTS staff_availability CASCADE;

DROP POLICY IF EXISTS tenant_isolation_shifts ON shifts;
DROP TABLE IF EXISTS shifts CASCADE;
