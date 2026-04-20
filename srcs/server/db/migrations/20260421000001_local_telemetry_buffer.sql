-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS local_telemetry_buffer (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    metric_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS local_telemetry_buffer;
-- +goose StatementEnd
