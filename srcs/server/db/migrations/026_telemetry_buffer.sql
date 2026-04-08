CREATE TABLE IF NOT EXISTS telemetry_buffer (
    id SERIAL PRIMARY KEY,
    metric_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Alter telemetry buffer for tenant isolation
ALTER TABLE telemetry_buffer ADD COLUMN organization_id TEXT DEFAULT 'system';
