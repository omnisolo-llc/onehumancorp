CREATE TABLE IF NOT EXISTS distributed_locks (
    lock_key VARCHAR(255) PRIMARY KEY,
    owner VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);
