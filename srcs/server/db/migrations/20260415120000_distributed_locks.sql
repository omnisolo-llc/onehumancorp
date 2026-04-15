-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS distributed_locks (
    lock_key TEXT PRIMARY KEY,
    owner_id TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS distributed_locks;
-- +goose StatementEnd
