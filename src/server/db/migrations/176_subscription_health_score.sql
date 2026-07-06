-- +goose Up
-- Migration 176: Add health_score and last_health_check_at to subscribers

ALTER TABLE subscribers ADD COLUMN IF NOT EXISTS health_score INTEGER DEFAULT 100;
ALTER TABLE subscribers ADD COLUMN IF NOT EXISTS last_health_check_at TIMESTAMPTZ;

-- +goose Down
ALTER TABLE subscribers DROP COLUMN IF EXISTS health_score;
ALTER TABLE subscribers DROP COLUMN IF EXISTS last_health_check_at;
