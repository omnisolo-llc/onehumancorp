-- +goose Up
CREATE TABLE IF NOT EXISTS offline_telemetry_buffer (
    id SERIAL PRIMARY KEY,
    metric_name TEXT NOT NULL,
    payload_bytes BYTEA NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- +goose Down
DROP TABLE IF EXISTS offline_telemetry_buffer;
