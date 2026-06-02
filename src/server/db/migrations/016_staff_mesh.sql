-- Staff Mesh and Terminal Auth

CREATE TABLE IF NOT EXISTS ohc_staff_members (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    phone_number TEXT,
    role TEXT NOT NULL, -- e.g., 'Cashier', 'Manager'
    hashed_pin TEXT,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_staff_tenant ON ohc_staff_members(tenant_id);

ALTER TABLE ohc_staff_members ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_staff_members ON ohc_staff_members;
CREATE POLICY tenant_isolation_ohc_staff_members
ON ohc_staff_members
USING (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ohc_timecard_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    staff_member_id TEXT NOT NULL REFERENCES ohc_staff_members(id),
    event_type TEXT NOT NULL, -- 'CLOCK_IN', 'CLOCK_OUT'
    client_timestamp TIMESTAMPTZ NOT NULL,
    server_timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    sync_id TEXT, -- For offline deduplication
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS idx_ohc_timecard_events_staff ON ohc_timecard_events(tenant_id, staff_member_id);

ALTER TABLE ohc_timecard_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_timecard_events ON ohc_timecard_events;
CREATE POLICY tenant_isolation_ohc_timecard_events
ON ohc_timecard_events
USING (tenant_id = current_setting('app.current_tenant', true));
