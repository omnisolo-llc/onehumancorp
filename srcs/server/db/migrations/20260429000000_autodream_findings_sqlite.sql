-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS autodream_findings (
    id TEXT PRIMARY KEY,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    content TEXT NOT NULL,
    embedding TEXT
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS autodream_findings;
-- +goose StatementEnd
