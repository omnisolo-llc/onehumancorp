-- +goose Up
-- Add health_score to subscriptions and subscribers tables for churn prediction

ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS health_score DOUBLE PRECISION DEFAULT 1.0;
ALTER TABLE subscribers ADD COLUMN IF NOT EXISTS health_score DOUBLE PRECISION DEFAULT 1.0;

-- +goose Down
ALTER TABLE subscriptions DROP COLUMN IF EXISTS health_score;
ALTER TABLE subscribers DROP COLUMN IF EXISTS health_score;
