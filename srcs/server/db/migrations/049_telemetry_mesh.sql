-- +goose Up
CREATE TABLE IF NOT EXISTS telemetry_buffer (
    id SERIAL PRIMARY KEY,
    metric_name TEXT NOT NULL,
    value REAL NOT NULL,
    labels_json TEXT,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    sync_status TEXT DEFAULT 'pending'
);

-- +goose Down
DROP TABLE IF EXISTS telemetry_buffer;
