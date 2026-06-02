-- Migration 061: Staff and Timecards
CREATE TABLE IF NOT EXISTS team_members (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    role TEXT NOT NULL,
    phone_number TEXT,
    hashed_pin TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS timecard_events (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    team_member_id TEXT NOT NULL REFERENCES team_members(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL, -- 'CLOCK_IN', 'CLOCK_OUT'
    client_timestamp TIMESTAMPTZ NOT NULL,
    synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    device_id TEXT
);

ALTER TABLE team_members ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_team_members ON team_members USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE timecard_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_timecard_events ON timecard_events USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
