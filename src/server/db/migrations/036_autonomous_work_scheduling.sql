-- Autonomous Work Scheduling Schema

-- Job Templates
CREATE TABLE IF NOT EXISTS job_templates (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    estimated_duration_minutes INTEGER NOT NULL,
    base_price_cents BIGINT NOT NULL DEFAULT 0,
    skills_required JSONB DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_job_templates_tenant_id ON job_templates(tenant_id);

ALTER TABLE job_templates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_job_templates ON job_templates;
CREATE POLICY tenant_isolation_job_templates
ON job_templates
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


-- Staff Profiles
CREATE TABLE IF NOT EXISTS staff_profiles (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    staff_member_id TEXT NOT NULL,
    work_hours JSONB DEFAULT '[]',
    skills JSONB DEFAULT '[]',
    current_location_lat DOUBLE PRECISION,
    current_location_lng DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_staff_profiles_tenant_id ON staff_profiles(tenant_id);

ALTER TABLE staff_profiles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_profiles ON staff_profiles;
CREATE POLICY tenant_isolation_staff_profiles
ON staff_profiles
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


-- Appointments
CREATE TABLE IF NOT EXISTS appointments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
    job_template_id TEXT NOT NULL REFERENCES job_templates(id),
    assigned_staff_id TEXT REFERENCES staff_profiles(id),
    scheduled_start_time TIMESTAMPTZ,
    scheduled_end_time TIMESTAMPTZ,
    actual_start_time TIMESTAMPTZ,
    actual_end_time TIMESTAMPTZ,
    location_lat DOUBLE PRECISION,
    location_lng DOUBLE PRECISION,
    location_address TEXT,
    status TEXT NOT NULL CHECK (status IN ('Requested', 'Scheduled', 'En-Route', 'In-Progress', 'Completed', 'Cancelled')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_appointments_tenant_id ON appointments(tenant_id);
CREATE INDEX IF NOT EXISTS idx_appointments_staff_status ON appointments(assigned_staff_id, status);

ALTER TABLE appointments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_appointments ON appointments;
CREATE POLICY tenant_isolation_appointments
ON appointments
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
