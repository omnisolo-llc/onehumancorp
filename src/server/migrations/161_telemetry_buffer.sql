CREATE TABLE IF NOT EXISTS telemetry_buffer (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    metric_name TEXT NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE telemetry_buffer ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Tenant isolation for telemetry_buffer"
    ON telemetry_buffer
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant'));
