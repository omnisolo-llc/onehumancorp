-- +goose Up
CREATE TABLE IF NOT EXISTS offline_telemetry_buffer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    metric_name TEXT NOT NULL,
    payload_bytes BLOB NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- +goose Down
DROP TABLE IF EXISTS offline_telemetry_buffer;
