-- +goose Up
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS language_preference TEXT DEFAULT 'English';

-- +goose Down
ALTER TABLE tenants DROP COLUMN IF EXISTS language_preference;
