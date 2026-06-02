-- +goose Up
ALTER TABLE raw_materials ADD COLUMN IF NOT EXISTS lead_time_days INT DEFAULT 7;
ALTER TABLE products ADD COLUMN IF NOT EXISTS lead_time_days INT DEFAULT 7;

-- +goose Down
ALTER TABLE raw_materials DROP COLUMN IF NOT EXISTS lead_time_days;
ALTER TABLE products DROP COLUMN IF NOT EXISTS lead_time_days;
