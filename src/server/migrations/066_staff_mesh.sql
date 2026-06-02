-- Create staff_members table
CREATE TABLE IF NOT EXISTS staff_members (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    role TEXT NOT NULL,
    pin_hash TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_staff_members_tenant ON staff_members(tenant_id);

ALTER TABLE staff_members ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_members ON staff_members;
CREATE POLICY tenant_isolation_staff_members ON staff_members USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Create timecard_events table
CREATE TABLE IF NOT EXISTS timecard_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    staff_member_id TEXT NOT NULL REFERENCES staff_members(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL, -- 'clock_in' or 'clock_out'
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_timecard_events_tenant_staff ON timecard_events(tenant_id, staff_member_id);
CREATE INDEX IF NOT EXISTS idx_timecard_events_tenant_time ON timecard_events(tenant_id, timestamp);

ALTER TABLE timecard_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_timecard_events ON timecard_events;
CREATE POLICY tenant_isolation_timecard_events ON timecard_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
