-- +goose Up
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS language_preference TEXT NOT NULL DEFAULT 'en';

-- +goose Down
ALTER TABLE tenants DROP COLUMN IF EXISTS language_preference;
