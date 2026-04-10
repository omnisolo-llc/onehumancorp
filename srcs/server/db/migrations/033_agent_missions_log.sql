-- +goose Up
-- Add mission_log column to agent_missions to track blockers and updates
ALTER TABLE agent_missions ADD COLUMN mission_log TEXT;

-- +goose Down
-- Remove mission_log column
-- ALTER TABLE agent_missions DROP COLUMN mission_log;
