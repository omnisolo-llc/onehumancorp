CREATE TABLE IF NOT EXISTS telemetry_buffer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    metric_name TEXT NOT NULL,
    metric_type TEXT NOT NULL,
    value REAL NOT NULL,
    labels_json TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    sync_status TEXT NOT NULL
);
