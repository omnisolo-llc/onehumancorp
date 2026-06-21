CREATE TABLE IF NOT EXISTS telemetry_buffer (
    id SERIAL PRIMARY KEY,
    tenant_id TEXT NOT NULL DEFAULT 'default_tenant',
    metric_name TEXT NOT NULL,
    metric_type TEXT NOT NULL,
    value REAL NOT NULL,
    labels_json TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    sync_status TEXT NOT NULL
);

ALTER TABLE telemetry_buffer ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
