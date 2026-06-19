-- +goose Up
-- Migration 135: Add source and agent_type columns to agent_action_requests

ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS source TEXT;
ALTER TABLE agent_action_requests ADD COLUMN IF NOT EXISTS agent_type TEXT;

-- +goose Down
ALTER TABLE agent_action_requests DROP COLUMN IF EXISTS source;
ALTER TABLE agent_action_requests DROP COLUMN IF EXISTS agent_type;
