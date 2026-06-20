-- +goose Up
-- Migration 141: Add triage fields to agent_feed_items

ALTER TABLE agent_feed_items
ADD COLUMN priority_score INTEGER DEFAULT 0,
ADD COLUMN correlation_id TEXT;

-- +goose Down
ALTER TABLE agent_feed_items
DROP COLUMN correlation_id,
DROP COLUMN priority_score;
