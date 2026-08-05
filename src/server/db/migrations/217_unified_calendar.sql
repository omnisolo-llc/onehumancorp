-- +goose Up
CREATE TABLE IF NOT EXISTS calendars (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    display_timezone TEXT NOT NULL DEFAULT 'UTC',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_calendars_tenant_id ON calendars(tenant_id);

ALTER TABLE calendars ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_calendars ON calendars;
CREATE POLICY tenant_isolation_calendars
ON calendars
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_calendars ON calendars;
DROP TABLE IF EXISTS calendars CASCADE;
