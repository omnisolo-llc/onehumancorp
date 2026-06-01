-- +goose Up
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS trial_days_left INT DEFAULT 14;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS twitter_shared BOOLEAN DEFAULT false;

-- +goose Down
ALTER TABLE tenants DROP COLUMN IF EXISTS trial_days_left;
ALTER TABLE tenants DROP COLUMN IF EXISTS twitter_shared;
