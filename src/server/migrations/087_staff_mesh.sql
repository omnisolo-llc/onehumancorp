-- Staff Management Mesh
-- GitHub Issue #22554
CREATE TABLE IF NOT EXISTS ohc_staff_member (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    role TEXT NOT NULL,
    pin_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_ohc_staff_member_tenant
ON ohc_staff_member(tenant_id);
ALTER TABLE ohc_staff_member ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_staff_member ON ohc_staff_member;
CREATE POLICY tenant_isolation_ohc_staff_member
ON ohc_staff_member
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE TABLE IF NOT EXISTS ohc_timecard_event (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    staff_id TEXT NOT NULL,
    event_type TEXT NOT NULL, -- CLOCK_IN, CLOCK_OUT
    event_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    sync_status TEXT NOT NULL DEFAULT 'SYNCED',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_ohc_timecard_event_tenant_staff
ON ohc_timecard_event(tenant_id, staff_id);
ALTER TABLE ohc_timecard_event ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_timecard_event ON ohc_timecard_event;
CREATE POLICY tenant_isolation_ohc_timecard_event
ON ohc_timecard_event
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));