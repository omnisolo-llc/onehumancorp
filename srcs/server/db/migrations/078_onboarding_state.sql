-- +goose Up
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS state TEXT DEFAULT '';

-- +goose Down
ALTER TABLE tenants DROP COLUMN IF EXISTS state;
