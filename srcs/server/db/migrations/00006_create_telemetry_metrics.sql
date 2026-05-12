-- +goose Up
CREATE TABLE IF NOT EXISTS local_telemetry_metrics (
    id TEXT PRIMARY KEY,
    metric_name TEXT NOT NULL,
    value REAL NOT NULL,
    attributes TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    synced_to_cloud INTEGER NOT NULL DEFAULT FALSE
);

CREATE INDEX IF NOT EXISTS idx_telemetry_synced ON local_telemetry_metrics(synced_to_cloud);

-- +goose Down
DROP TABLE IF EXISTS local_telemetry_metrics;
