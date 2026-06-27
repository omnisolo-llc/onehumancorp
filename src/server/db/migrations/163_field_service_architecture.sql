-- +goose Up

CREATE TABLE IF NOT EXISTS unified_booking_resources (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    resource_type TEXT NOT NULL, -- e.g., 'staff', 'equipment', 'vehicle'
    name TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE unified_booking_resources ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_unified_booking_resources
ON unified_booking_resources
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE quotes ADD COLUMN IF NOT EXISTS service_id TEXT;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS proposed_slot_id TEXT;

-- +goose Down
ALTER TABLE quotes DROP COLUMN IF EXISTS proposed_slot_id;
ALTER TABLE quotes DROP COLUMN IF EXISTS service_id;
DROP POLICY IF EXISTS tenant_isolation_unified_booking_resources ON unified_booking_resources;
DROP TABLE IF EXISTS unified_booking_resources CASCADE;
