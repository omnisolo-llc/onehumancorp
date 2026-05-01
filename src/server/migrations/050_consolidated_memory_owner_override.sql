-- +goose Up
ALTER TABLE consolidated_memory ADD COLUMN IF NOT EXISTS owner_override BOOLEAN DEFAULT FALSE;

-- +goose Down
ALTER TABLE consolidated_memory DROP COLUMN IF EXISTS owner_override;
