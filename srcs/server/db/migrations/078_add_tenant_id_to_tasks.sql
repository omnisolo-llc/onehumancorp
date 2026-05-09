-- +goose Up
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';

-- +goose Down
ALTER TABLE tasks DROP COLUMN IF EXISTS tenant_id;
