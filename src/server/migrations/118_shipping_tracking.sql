CREATE TABLE IF NOT EXISTS tracking_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    tracking_number TEXT NOT NULL,
    status TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_tracking_events_tenant ON tracking_events(tenant_id);
CREATE INDEX IF NOT EXISTS idx_tracking_events_tracking_number ON tracking_events(tracking_number);

ALTER TABLE tracking_events ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = 'public'
        AND tablename = 'tracking_events'
        AND policyname = 'tenant_isolation_tracking_events'
    ) THEN
        CREATE POLICY tenant_isolation_tracking_events ON tracking_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;
