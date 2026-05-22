CREATE TABLE IF NOT EXISTS local_telemetry_buffer (
    id SERIAL PRIMARY KEY,
    metric_name TEXT NOT NULL,
    metric_type TEXT NOT NULL,
    value REAL NOT NULL,
    labels_json TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    sync_status TEXT NOT NULL
);
