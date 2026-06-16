-- 1. Locations
CREATE TABLE IF NOT EXISTS locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_locations_tenant ON locations(tenant_id);
ALTER TABLE locations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_locations ON locations;
CREATE POLICY tenant_isolation_locations ON locations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 2. Role Assignment
CREATE TABLE IF NOT EXISTS location_role_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    location_id UUID NOT NULL REFERENCES locations(id),
    role TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_location_role_assignments_tenant ON location_role_assignments(tenant_id);
CREATE INDEX IF NOT EXISTS idx_location_role_assignments_location ON location_role_assignments(location_id);
ALTER TABLE location_role_assignments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_location_role_assignments ON location_role_assignments;
CREATE POLICY tenant_isolation_location_role_assignments ON location_role_assignments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 3. Escalations
CREATE TABLE IF NOT EXISTS escalations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    summary TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_escalations_tenant ON escalations(tenant_id);
CREATE INDEX IF NOT EXISTS idx_escalations_task ON escalations(task_id);
ALTER TABLE escalations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_escalations ON escalations;
CREATE POLICY tenant_isolation_escalations ON escalations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 4. Alter tasks to add location_id
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS location_id TEXT;
CREATE INDEX IF NOT EXISTS idx_shared_tasks_location_id ON shared_tasks(location_id);
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS location_id TEXT;
CREATE INDEX IF NOT EXISTS idx_swarm_tasks_location_id ON swarm_tasks(location_id);
