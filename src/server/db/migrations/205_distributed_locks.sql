-- +goose Up
CREATE TABLE IF NOT EXISTS distributed_locks (
    id TEXT PRIMARY KEY,
    lock_val TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

-- +goose Down
DROP TABLE IF EXISTS distributed_locks;
