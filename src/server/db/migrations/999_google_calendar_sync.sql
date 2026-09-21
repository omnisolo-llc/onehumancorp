-- +goose Up
CREATE TABLE IF NOT EXISTS google_calendar_sync_mappings (
    booking_id TEXT NOT NULL REFERENCES bookings(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    calendar_id TEXT NOT NULL,
    external_event_id TEXT,
    sync_state TEXT NOT NULL CHECK (sync_state IN ('pending', 'synced', 'delete_pending', 'deleted', 'failed')),
    last_synced_at TIMESTAMPTZ,
    last_sync_error TEXT,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (booking_id, provider)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_gcal_sync_ext ON google_calendar_sync_mappings (provider, calendar_id, external_event_id) WHERE external_event_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS google_calendar_watch_channels (
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    channel_id TEXT PRIMARY KEY,
    resource_id TEXT NOT NULL,
    expiration TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE google_calendar_sync_mappings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_gcal_sync
ON google_calendar_sync_mappings
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE google_calendar_watch_channels ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_gcal_watch
ON google_calendar_watch_channels
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_gcal_watch ON google_calendar_watch_channels;
DROP TABLE IF EXISTS google_calendar_watch_channels CASCADE;

DROP POLICY IF EXISTS tenant_isolation_gcal_sync ON google_calendar_sync_mappings;
DROP TABLE IF EXISTS google_calendar_sync_mappings CASCADE;
