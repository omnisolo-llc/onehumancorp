-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS distributed_locks (
    key VARCHAR(255) PRIMARY KEY,
    token VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS distributed_locks;
-- +goose StatementEnd
