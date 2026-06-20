-- +goose Up
-- Migration 141: Add Work Triage Engine capabilities to agent_feed_items

ALTER TABLE agent_feed_items ADD COLUMN IF NOT EXISTS correlation_id TEXT;
ALTER TABLE agent_feed_items ADD COLUMN IF NOT EXISTS priority_score INTEGER DEFAULT 0;

-- +goose Down
ALTER TABLE agent_feed_items DROP COLUMN IF EXISTS correlation_id;
ALTER TABLE agent_feed_items DROP COLUMN IF EXISTS priority_score;
