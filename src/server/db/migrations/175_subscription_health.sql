-- +goose Up
-- Migration 175: Add health_score and last_engagement_at to subscribers

ALTER TABLE subscribers ADD COLUMN IF NOT EXISTS health_score INTEGER NOT NULL DEFAULT 100;
ALTER TABLE subscribers ADD COLUMN IF NOT EXISTS last_engagement_at TIMESTAMPTZ;

-- +goose Down
ALTER TABLE subscribers DROP COLUMN IF EXISTS health_score;
ALTER TABLE subscribers DROP COLUMN IF EXISTS last_engagement_at;
