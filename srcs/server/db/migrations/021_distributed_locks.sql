-- 021_distributed_locks.sql
-- Table for robust distributed locking when Redis is unavailable (Standalone/Postgres fallback)

CREATE TABLE IF NOT EXISTS distributed_locks (
    lock_key VARCHAR PRIMARY KEY,
    owner_id VARCHAR NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
