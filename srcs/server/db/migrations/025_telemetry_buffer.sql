CREATE TABLE IF NOT EXISTS telemetry_buffer (
    id TEXT PRIMARY KEY,
    metric_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
