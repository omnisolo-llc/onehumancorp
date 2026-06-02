CREATE TABLE IF NOT EXISTS staff_members (
    id VARCHAR(255) PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    phone_number VARCHAR(255) NOT NULL,
    role VARCHAR(255) NOT NULL,
    hashed_pin VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE staff_members ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_staff_members ON staff_members
    USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS timecard_events (
    id VARCHAR(255) PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    staff_member_id VARCHAR(255) NOT NULL REFERENCES staff_members(id) ON DELETE CASCADE,
    event_type VARCHAR(50) NOT NULL, -- 'CLOCK_IN', 'CLOCK_OUT'
    occurred_at TIMESTAMP WITH TIME ZONE NOT NULL,
    synced_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_offline_sync BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE timecard_events ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_timecard_events ON timecard_events
    USING (tenant_id = current_setting('app.current_tenant', true));
