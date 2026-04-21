CREATE TABLE IF NOT EXISTS telemetry_buffer (
    id TEXT PRIMARY KEY,
    metric_name TEXT NOT NULL,
    value REAL NOT NULL,
    labels_json TEXT,
    timestamp DATETIME NOT NULL,
    sync_status TEXT DEFAULT 'pending'
);
