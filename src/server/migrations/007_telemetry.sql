CREATE TABLE IF NOT EXISTS telemetry_buffer (
    id SERIAL PRIMARY KEY,
    metric_name TEXT NOT NULL,
    metric_type TEXT NOT NULL,
    value REAL NOT NULL,
    labels_json TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    sync_status TEXT NOT NULL
);
ALTER TABLE telemetry_buffer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer USING (tenant_id::text = current_setting('app.current_tenant', true));
