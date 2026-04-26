-- 029_telemetry_buffer.sql

CREATE TABLE IF NOT EXISTS telemetry_buffer (
    id SERIAL PRIMARY KEY,
    metric_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
