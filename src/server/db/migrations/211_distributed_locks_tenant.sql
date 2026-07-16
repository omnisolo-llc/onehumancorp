-- +goose Up
ALTER TABLE distributed_locks ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT 'system';
ALTER TABLE distributed_locks DROP CONSTRAINT IF EXISTS distributed_locks_pkey;
ALTER TABLE distributed_locks ADD PRIMARY KEY (id, tenant_id);

-- +goose Down
ALTER TABLE distributed_locks DROP CONSTRAINT IF EXISTS distributed_locks_pkey;
ALTER TABLE distributed_locks ADD PRIMARY KEY (id);
ALTER TABLE distributed_locks DROP COLUMN IF EXISTS tenant_id;
