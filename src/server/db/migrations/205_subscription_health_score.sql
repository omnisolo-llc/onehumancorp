-- +goose Up
-- Add health_score and last_engagement_at to subscriptions
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS health_score INTEGER DEFAULT 100;
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS last_engagement_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;

-- Add health_score and last_engagement_at to subscribers
ALTER TABLE subscribers ADD COLUMN IF NOT EXISTS health_score INTEGER DEFAULT 100;
ALTER TABLE subscribers ADD COLUMN IF NOT EXISTS last_engagement_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;

-- +goose Down
ALTER TABLE subscriptions DROP COLUMN IF EXISTS health_score;
ALTER TABLE subscriptions DROP COLUMN IF EXISTS last_engagement_at;
ALTER TABLE subscribers DROP COLUMN IF EXISTS health_score;
ALTER TABLE subscribers DROP COLUMN IF EXISTS last_engagement_at;
